import argparse 

def create_parser():
    parent_parser = argparse.ArgumentParser(add_help=False)
    parent_parser.add_argument("-d", "--debug", action="store_true", default=argparse.SUPPRESS)

    main_parser = argparse.ArgumentParser(description = "The stupidest content tracker")
    argsubparsers = main_parser.add_subparsers(title="Commands", dest="command")
    argsubparsers.required = True

    main_parser.add_argument("-d", "--debug", action="store_true", help="Enable debug logging")

    def add_command(name, help):
        """Creates a sub-parser that automatically inherits the debug flag."""
        return argsubparsers.add_parser(name, parents=[parent_parser], help=help)

    argsp = add_command("init", help="Initialize a new, empty repository.")
    argsp.add_argument("path",
                    metavar="directory",
                    nargs="?",
                    default=".",
                    help="Where to create the repository.")

    argsp = add_command("cat-file", help="Provide content of repository objects")
    argsp.add_argument("type",
                    metavar="type",
                    choices=["blob", "commit", "tag", "tree"],
                    help="Specify the type")
                    
    argsp.add_argument("object",
                    metavar="object",
                    help="The object to display")
    
    argsp = add_command("hash-object", help="Compute object ID and optionally creates a blob from a file")
    
    argsp.add_argument("-t",
                    metavar="type",
                    dest="type",
                    choices=["blob", "commit", "tag", "tree"],
                    default="blob",
                    help="Specify the type")
    
    argsp.add_argument("-w",
                    dest="write",
                    action="store_true",
                    help="Actually write the object into the database")

    argsp.add_argument("path",
                    help="Read object from <file>")

    argsp = add_command("log", help="Display history of a given commit.")
    argsp.add_argument("commit",
                    default="HEAD",
                    nargs="?",
                    help="Commit to start at.")

    argsp = add_command("ls-tree", help="Pretty-print a tree object.")
    argsp.add_argument("-r",
                    dest="recursive",
                    action="store_true",
                    help="Recurse into sub-trees")

    argsp.add_argument("tree",
                    help="A tree-ish object.")
    
    argsp = add_command("checkout", help="Checkout a commit inside of a directory.")
    argsp.add_argument("commit",
                    help="The commit or tree to checkout.")

    argsp.add_argument("path",
                    help="The EMPTY directory to checkout on.")

    argsp = add_command("show-ref", help="List references.")

    argsp = add_command("tag", help="List and create tags")

    argsp.add_argument("-a",
                    action="store_true",
                    dest="create_tag_object",
                    help="Whether to create a tag object")

    argsp.add_argument("name",
                    nargs="?",
                    help="The new tag's name")

    argsp.add_argument("object",
                    default="HEAD",
                    nargs="?",
                    help="The object the new tag will point to")
    
    argsp = add_command("rev-parse", help="Parse revision (or other objects) identifiers")

    argsp.add_argument("--type",
                    metavar="type",
                    dest="type",
                    choices=["blob", "commit", "tag", "tree"],
                    default=None,
                    help="Specify the expected type")

    argsp.add_argument("name",
                    help="The name to parse")

    argsp = add_command("ls-files", help = "List all the stage files")
    argsp.add_argument("--verbose", action="store_true", help="Show everything.")

    argsp = add_command("check-ignore", help = "Check path(s) against ignore rules.")
    argsp.add_argument("path", nargs="+", help="Paths to check")
    
    return main_parser

