import os

from hash import object_read
from init import repo_find

from ref import object_find
from utils import silent
import logging
logger = logging.getLogger(__name__)

def cmd_ls_tree(args):
    repo = repo_find()
    ls_tree(repo, args.tree, args.recursive)

def ls_tree(repo, ref, recursive, prefix = ""):
    hash = object_find(repo, ref, b'tree')
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




# try:
#     # --- Your silent code block ---
#     for item in tree.data:
#         try:
#             object_read(repo, item.hash)
#         except:
#             pass
#     # ------------------------------
# finally:
#     # 3. Always restore the level, even if an error occurred
#     logger.setLevel(old_level)

def checkout_tree(repo, tree, base_path):
    logger.debug("Tree contents : ")
    if False:
        logger.debug(f'{'mode':9} {'hash':40} exists path')
        for item in tree.data:
            flag = False

            previous_level = logging.root.manager.disable
            logging.disable(logging.DEBUG)
            try:
                object_read(repo, item.hash)
                flag = True
            except:
                pass
            logging.disable(previous_level)
            # logging.disable(logging.NOTSET)
            
            
            # logger.debug(f'{item.mode} {item.hash} {item.path}')
            logger.debug(f'{item.mode} {item.hash} {flag:6} {item.path}')
            # logger.debug(item.mode, item.hash, item.path)
        raise ZeroDivisionError

    else:

        logger.debug(f'{'mode':9} {'hash':40} path')
        # logger.debug(f'{'mode':9} {'hash':40} exists path')
        for item in tree.data:
            # flag = False

            # old_level = logger.getEffectiveLevel()
            # logger.setLevel(logging.CRITICAL)
            # logging.disable(logging.DEBUG)
            # try:
            #     object_read(repo, item.hash)
            #     flag = True
            # except:
            #     pass
            # # logger.setLevel(old_level)
            # logging.disable(logging.NOTSET)
            
            
            logger.debug(f'{item.mode} {item.hash} {item.path}')
            # logger.debug(f'{item.mode} {item.hash} {flag:6} {item.path}')
            # logger.debug(item.mode, item.hash, item.path)
    for item in tree.data:
        logger.debug("reading item hash: %s",item.hash)

        # previous_level = logging.root.manager.disable
        # logging.disable(logging.DEBUG)
        # try:
        with silent():
            obj = object_read(repo, item.hash)
        # except:
        #     pass
        # logging.disable(previous_level)
        # logging.disable(logging.NOTSET)
        # obj = object_read(repo, item.hash)
        path = os.path.join(base_path, item.path)
        if obj.fmt == b'tree':
            os.mkdir(path)
            checkout_tree(repo, obj, path)
        elif obj.fmt == b'blob':

            with open(path, 'wb') as f:
                f.write(obj.data)
            logger.debug("\tWrote blob: %s",path)
        else:
            pass
            # raise Exception(f"Invalid type found : {obj.fmt}")

def cmd_checkout(args):
    repo = repo_find()
    obj = object_read(repo, object_find(repo, args.commit))
    if obj.fmt == b'commit':
        hash = (obj.data[b'tree'][-1]).decode('ascii')
        logger.debug("found tree hash : %s",hash)

        obj = object_read(repo, object_find(repo, hash))
        logger.debug("loaded obj of type : %s",obj.fmt)
    # else


    assert obj.fmt == b'tree', f'Unexpected object type : {obj.fmt}'

    path = args.path
    if os.path.isdir(path):
        if not os.path.isdir(path):
            raise Exception(f"Not a directory {path}!")
        elif os.listdir(path):
            raise Exception(f"Not empty {path}!")
    else:
        os.makedirs(path)

    checkout_tree(repo, obj, os.path.realpath(path))
