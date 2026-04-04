import sys
import logging
from args import create_parser
from init import cmd_init
from hash import cmd_cat_file, cmd_hash_object
from log import cmd_log
from checkout import cmd_ls_tree

def main(argv=sys.argv[1:]):

    # If no arguments, or first argument is "help", show help
    if len(argv)==0 or argv[0] == "help":
        argv = ["-h"]
    
    argparser = create_parser()
    args = argparser.parse_args(argv)
    if args.debug:
        # Show everything DEBUG and above
        logging.basicConfig(level=logging.DEBUG, format='%(levelname)s [%(name)s]: %(message)s')

        logger = logging.getLogger(__name__)
        logger.debug("Args are : %s", args)
    else:
        # Show only INFO and above (effectively hides DEBUG)
        logging.basicConfig(level=logging.INFO, format='%(message)s')

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


