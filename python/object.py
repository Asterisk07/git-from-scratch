import hashlib
import os
import zlib 
from abc import ABC, abstractmethod
import logging

from init import repo_file, repo_find
from utils import cat
# from commit import GitCommit
# from commit import read_metadata, write_metadata

logger = logging.getLogger(__name__)

class GitObject(ABC):
    # def __init__(self, data = None):
    #     self.data = data

    def __init__(self, data=None):
        if data != None:
            self.load(data)
        else:
            self.init()

    @abstractmethod
    def dump(self):
        pass

    @abstractmethod
    def load(self,data):
        pass

class GitBlob(GitObject):
    fmt=b'blob'

    def dump(self):
        return self.data

    def load(self,data):
        self.data = data



class GitCommit(GitObject):
    fmt = b'commit'

    def init(self):
        self.data = {}

    def dump(self):
        return write_metadata(self.data)
    
    def load(self,data):
        # raise ExceptionError
        # self.data = read_metadata(data)
        val = read_metadata(data)
        assert(type(val) == type({}))
        # print("-"*10)
        # print(val)
        # print("-"*10)
        self.data = val


def read_metadata(text):
    data = {}
    message_flag = 0
    data[None] = []
    SPACE = b' '
    NEWLINE = b'\n'

    start = 0
    end = 0

    while True:
        end = text.find(NEWLINE, start)
        if end < 0:
            break
        line = text[start:end]
        start = end + 1
        
            
        if message_flag:
            data[None].append(line)
            continue

        space_idx = line.find(SPACE)
        if space_idx < 0 :
            message_flag = 1
        elif space_idx == 0:
            data[key][-1] +=  line[space_idx+1:] + NEWLINE
        else:
            key = line[:space_idx]
            value = line[space_idx+1:] + NEWLINE
            if key not in data:
                data[key] = []
            data[key].append(value)
            
    data[None] = NEWLINE.join(data[None])
    return data

def write_metadata(data):
    text = b''
    SPACE = b' '
    NEWLINE = b'\n'

    for key in data:
        if key is None:
            continue
        val_list = data[key]
        for val in val_list:
            text += key + SPACE + val.replace(NEWLINE, NEWLINE + SPACE).rstrip(SPACE)
    text += NEWLINE + data[None]
    return text

def object_read(repo, hash):
    logger.debug("searching at : %s and %s", hash[:2], hash[2:])
    path = repo_file(repo,"objects", hash[:2], hash[2:])
    logger.debug("im the path : %s",path)
    if path is not None:
       path = path.rstrip('\n')

    # if not os.path.isfile(path):
        # print("INVALID PATH : " , path)
    # print("CHECKING PATH ", path)
    # assert(os.path.isfile(path))
    with open(path,'rb') as f:
        raw = zlib.decompress(f.read())
    
    x = raw.find(b' ')
    y = raw.find(b'\x00')
    type1 = raw[:x]
    size = int(raw[x+1:y].decode("ascii"))
    assert(size == len(raw) - y-1)
    # print(type1)
    # raise Index

    match type1:
        case b'blob' : c = GitBlob
        case b'commit' : c = GitCommit
        case b'tag' : c = GitTag
        case b'tree' : c = GitTree
        case _:
            raise Exception(f"Unknown type : {type1} for hash : {hash}")

    content = raw[y+1:]
    # print(content)
    # raise ZeroDivisionError
    # print( b'commit' == type1)
    # print("-"*10)
    # print(content)
    # print("-"*10)
    # raise Index
    return c(content)
    # return

def get_hash(obj):
    content = obj.dump()
    content = obj.fmt + b' ' + str(len(content)).encode() + b'\x00' + content
    hash = hashlib.sha1(content).hexdigest()
    return hash, content

def object_write(obj, repo):
    hash, content = get_hash(obj)
    logger.debug("searching at : %s and %s", hash[:2], hash[2:])
    if repo:
        path = repo_file(repo,"objects", hash[:2], hash[2:], mkdir = True)

        with open(path,'wb') as f:
            f.write(zlib.compress(content))
    return hash

def object_find(repo, name, fmt=None, follow=True):
    return name

def cat_file(repo, obj, fmt=None):
    obj = object_read(repo, object_find(repo, obj, fmt=fmt))
    cat(obj.dump())

def cmd_cat_file(args):
    repo = repo_find()
    logger.debug("found repo at : %s",repo.gitdir)
    cat_file(repo, args.object, fmt=args.type.encode())

def object_hash(f, type1, repo = None):
    data = f.read()
    logger.debug("received type : %s",type1)
    match type1:
        case b'blob' : c = GitBlob
        case b'commit' : c = GitCommit
        case b'tag' : c = GitTag
        case b'tree' : c = GitTree
        case _:
            raise Exception(f"Unknown type : {type1} for hash : {hash}")
    
    obj = c(data)
    return object_write(obj, repo)

def cmd_hash_object(args):
    repo = None
    if args.write:
        repo = repo_find()
    
    with open(args.path, 'rb') as f:
        logger.debug("Type is : %s",args.type)
        hash = object_hash(f, args.type.encode(), repo)
        print(hash)