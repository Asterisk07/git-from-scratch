import argparse, configparser, grp, pwd, hashlib, os, re, sys, zlib 
from datetime import datetime
from fnmatch import fnmatch
from math import ceil

argparser = argparse.ArgumentParser(description = "The stupidest content tracker")
argsubparsers = argparser.add_subparsers(title="Commands", dest="command")
argsubparsers.required = True

argsp = argsubparsers.add_parser("init", help="Initialize a new, empty repository.")
argsp.add_argument("path",
                   metavar="directory",
                   nargs="?",
                   default=".",
                   help="Where to create the repository.")

def main(argv=sys.argv[1:]):

    # If no arguments, or first argument is "help", show help
    if len(argv)==0 or argv[0] == "help":
        argv = ["-h"]
        
    args = argparser.parse_args(argv)
    match args.command:
        case "add"          : cmd_add(args)
        case "cat-file"     : cmd_cat_file(args)
        case "check-ignore" : cmd_check_ignore(args)
        case "checkout"     : cmd_checkout(args)
        case "commit"       : cmd_commit(args)
        case "hash-object"  : cmd_hash_object(args)
        case "init"         : cmd_init(args)
        case "log"          : cmd_log(args)
        case "ls-files"     : cmd_ls_files(args)
        case "ls-tree"      : cmd_ls_tree(args)
        case "rev-parse"    : cmd_rev_parse(args)
        case "rm"           : cmd_rm(args)
        case "show-ref"     : cmd_show_ref(args)
        case "status"       : cmd_status(args)
        case "tag"          : cmd_tag(args)
        case _              : print("Bad command.")

class Repository:
    _git_name = '.git'
    _ini_name = 'config'
    def __init__(self, path1, force = False) : 
        self.worktree = path1
        self.gitdir = os.path.join(self.worktree, self._git_name)
        self.config_path =  os.path.join(self.gitdir, self._ini_name)
        self.config = configparser.ConfigParser(strict = False) # strict = False to allow duplicates, since git allows duplicates in config
        if not force:
            self.valid_check()

    def valid_check(self):
        if not(os.path.exists(self.gitdir)):
            raise Exception(f"Not a Git repository {path}")
        if not(os.path.exists(self.config_path)):
            raise Exception("ERROR : .git/config doesnt exist")
        try:
            self.config.read(self.config_path)
            val = self.config['core']['repositoryformatversion']
            val = int(val)
        except Exception as e:
            raise Exception(f"ERROR : Could not read config file : {e}")
        if val!=0:
            raise Exception(f"ERROR : Invalid value forrepositoryformatversion value : {val}")

def repo_path(repo, *path):
    """Compute path under repo's gitdir."""
    return os.path.join(repo.gitdir, *path)

def repo_file(repo, *path, mkdir = False):
    # create path upto the file
    if len(path) == 1 or repo_dir(repo, *path[-1], mkdir = mkdir):
        return repo_path(repo, *path)

def repo_dir(repo, *path, mkdir = False):
    # create path upto dir
    path1 = repo_path(repo, *path)
    if os.path.exists(path1):
        if os.path.isdir(path1):
            return path1
        else:
            raise Exception("ERROR Not a directory")
    
    if mkdir == True:
        os.makedirs(path1)
        return path1

def repo_create(path):
    # create a new repo at this path
    repo = Repository(path, force = True)
    if os.path.exists(repo.worktree):
        assert(os.path.isdir(repo.worktree))
        assert(not(os.path.exists(repo.gitdir) and os.listdir(repo.gitdir)))
    else:
        os.makedirs(repo.worktree)

    assert(repo_dir(repo,"branches",mkdir = True))
    assert(repo_dir(repo,"objects",mkdir = True))
    assert(repo_dir(repo,"refs","tags",mkdir = True))
    assert(repo_dir(repo,"refs","heads",mkdir = True))

    with open(repo_file(repo,"HEAD"),"w") as f:
        f.write("ref : refs/heads/master\n")
    with open(repo_file(repo,"description"),"w") as f:
        f.write("Unnamed repo, edit description to name it\n")
    with open(repo_file(repo,"config"),"w") as f:
        config = repo_default_config()
        config.write(f)
        
    return repo

def repo_default_config():
    con = configparser.ConfigParser()
    con.add_section("core")
    con.set("core","repositoryformatversion","0")
    con.set("core","filemode","false")
    con.set("core","bare","false")
    return con

def cmd_init(args):
    repo_create(args.path)

def repo_find(path = '.', required = True):
    path = os.path.realpath(path)

    if os.path.isdir(os.path.join(path, '.git')):
        return Repository(path)

    parent = os.path.dirname(path)

    if parent == path:
        if required == "True":
            raise Exception("ERROR couldnt find any git repo")

    return repo_find(parent, required)

