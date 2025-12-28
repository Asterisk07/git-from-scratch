import hashlib
import os
import zlib 
from abc import ABC, abstractmethod
from init import repo_file, repo_find
from utils import cat

class GitObject(ABC):
    def __init__(self, data = None):
        self.data = data

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

def object_read(repo, hash):
    # print("searching at : ",hash[:2],"and ",hash[2:])
    path = repo_file(repo,"objects", hash[:2], hash[2:])
    # print("im the path : ",path)
    assert(os.path.isfile(path))
    with open(path,'rb') as f:
        raw = zlib.decompress(f.read())
    
    x = raw.find(b' ')
    y = raw.find(b'\x00')
    type1 = raw[:x]
    size = int(raw[x+1:y].decode("ascii"))
    assert(size == len(raw) - y-1)

    match type1:
        case b'blob' : c = GitBlob
        case b'commit' : c = GitCommit
        case b'tag' : c = GitTag
        case b'tree' : c = GitTree
        case _:
            raise Exception(f"Unknown type : {type1} for hash : {hash}")

    content = raw[y+1:]
    return c(content)

def get_hash(obj):
    content = obj.dump()
    content = obj.fmt + b' ' + str(len(content)).encode() + b'\x00' + content
    hash = hashlib.sha1(content).hexdigest()
    return hash, content

def object_write(obj, repo):
    hash, content = get_hash(obj)
    print("searching at : ",hash[:2],"and ",hash[2:])
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
    # print("found repo at : ",repo.gitdir)
    cat_file(repo, args.object, fmt=args.type.encode())

def object_hash(f, type1, repo = None):
    data = f.read()
    print("received type : ",type1)
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
        print("Type is : ",args.type)
        hash = object_hash(f, args.type.encode(), repo)
        print(hash)