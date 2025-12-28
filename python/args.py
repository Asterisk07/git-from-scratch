import argparse 

def create_parser():
    argparser = argparse.ArgumentParser(description = "The stupidest content tracker")
    argsubparsers = argparser.add_subparsers(title="Commands", dest="command")
    argsubparsers.required = True

    argsp = argsubparsers.add_parser("init", help="Initialize a new, empty repository.")
    argsp.add_argument("path",
                    metavar="directory",
                    nargs="?",
                    default=".",
                    help="Where to create the repository.")

    return argparser

