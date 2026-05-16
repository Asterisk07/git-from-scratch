from models import *

import hashlib
import zlib 

from init import repo_file, repo_find

import os

import logging
logger = logging.getLogger(__name__)


def object_read(repo, hash) -> GitObject:
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

        if not os.path.exists(path):
            with open(path,'wb') as f:
                f.write(zlib.compress(content))
    return hash

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