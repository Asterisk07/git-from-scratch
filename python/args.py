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
    
    return main_parser

