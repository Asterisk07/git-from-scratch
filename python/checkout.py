import os

from hash import object_find, object_read
from init import repo_find

import logging
logger = logging.getLogger(__name__)

def cmd_ls_tree(args):
    repo = repo_find()
    ls_tree(repo, args.tree, args.recursive)

def ls_tree(repo, ref, recursive, prefix = ""):
    hash = object_find(repo, ref, recursive)
    logger.debug("found hash : %s",hash)

    tree = object_read(repo, hash)

    for item in tree.data:
        # if len(item.mode) == 
        mode = item.mode
        if len(item.mode) == 5:
            type = item.mode[:1]
        else:
            type = item.mode[:2]

        
        match type: # Determine the type.
            case b'04': type = "tree"
            case b'10': type = "blob" # A regular file.
            case b'12': type = "blob" # A symlink. Blob contents is link target.
            case b'16': type = "commit" # A submodule
            case _: raise Exception(f"Weird tree leaf mode {item.mode}")

        if not (recursive and type=='tree'): # This is a leaf
            print(f"{'0' * (6 - len(item.mode)) + item.mode.decode('ascii')} {type} {item.hash}\t{os.path.join(prefix, item.path)}")
        else: # This is a branch, recurse
            ls_tree(repo, item.hash, recursive, os.path.join(prefix, item.path))
