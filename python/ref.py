from init import repo_file, repo_find, repo_dir
import os

from utils import silent
import logging
logger = logging.getLogger(__name__)

REF_PREFIX = 'ref: '
def ref_resolve(repo, ref):
    path = repo_file(repo, ref)

    # Sometimes, an indirect reference may be broken.  This is normal
    # in one specific case: we're looking for HEAD on a new repository
    # with no commits.  In that case, .git/HEAD points to "ref:
    # refs/heads/main", but .git/refs/heads/main doesn't exist yet
    # (since there's no commit for it to refer to).

    if not os.path.isfile(path):
        return None

    with open(path) as f:
        data = f.read().rstrip('\n')

    if data.startswith(REF_PREFIX):
        return ref_resolve(repo, data.lstrip(REF_PREFIX))
    else:
        return data

def ref_list(repo, path = None):
    if path is None:
        path = repo_dir(repo, 'refs')

    ret = {}
    for item in sorted(os.listdir(path)):
        item_path  = os.path.join(path, item)
        if os.path.isdir(item_path):
            ret[item] = ref_list(repo, item_path)
        else:
            ret[item] = ref_resolve(repo, item_path)

    return ret

def show_ref(repo , refs, with_hash = True, prefix = ''):
    if prefix != '':
        prefix += '/'

    for item in refs:
        val = refs[item]

        if type(val) != str:

            logger.debug(f"k : v  = {item}:{val}")
            logger.debug("type of item is: %s",type(val))
            show_ref(repo, val, with_hash, prefix+item)

        elif with_hash:
            print(f"{val} {prefix}{item}")
        else:
            print(f"{prefix}{item}")

def cmd_show_ref(args):
    repo = repo_find()
    refs = ref_list(repo)
    show_ref(repo, refs, prefix = 'refs')


