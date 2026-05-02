from hash import logger, object_read
from init import repo_find
from ref import object_find
from utils import cat

def cat_file(repo, obj, fmt=None):
    obj = object_read(repo, object_find(repo, obj, fmt=fmt))
    print(type(obj), "is type obj ")
    cat(obj.dump())

def cmd_cat_file(args):
    repo = repo_find()
    logger.debug("found repo at : %s",repo.gitdir)
    cat_file(repo, args.object, fmt=args.type.encode())