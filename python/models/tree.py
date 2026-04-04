from .object import GitObject

class GitTree(GitObject):
    fmt = b'tree'

    def init(self):
        self.data = list

    def dump(self):
        return dump_tree(self.data)
    
    def load(self,data):
        self.data = load_tree(data)

class GitTreeLeaf ():
    def __init__(self, mode : bytes, path : str, hash : str):
        self.mode = mode
        self.path = path
        self.hash = hash

SPACE = b' '
NULL = b'\x00'
ZERO = b'0'


def load_tree_helper(text, start):
    x = text.find(SPACE, start)
    mode = text[start : x]
    if len(mode) == 5:
        mode = ZERO + mode
    assert(len(mode) == 6)

    y = text.find(NULL, x+1)
    path = text[x+1: y]
    path = path.decode("utf8")

    len_hash_bytes = 20
    len_hash_hex = 40 
    #one hex digit has 4 bits, so half a byte, so one byte is 2 hex digits

    end = y+1+len_hash_bytes
    hash = text[y+1 :  end]
    hash = int.from_bytes(hash, "big")
    hash = format(hash, f'0{len_hash_hex}x')
    
    return end, GitTreeLeaf(mode, path, hash)

def load_tree(tree):
    data_list = []
    start = 0
    end = len(tree)

    while start < end:
        start , data = load_tree_helper(tree, start)
        data_list.append(data)
  
    return data_list

def dump_tree_helper(leaf):

    # delimiter
    # basically we want that fooX (X is the delimter for a folder, here fooX is the folder and rest all are files)
    # should be such that the sort order is :
    # foo, foo.c, fooX,  fool
    # so X shud be more than . but less than any number or alphabet

    if leaf.mode.startswith(b'4'):
        return leaf.path + '/'
    else:
        return leaf.path

def dump_tree(tree):
    tree.data.sort(key = dump_tree_helper)
    text = b''
    for i in tree.data:
        text += i.mode
        text += SPACE
        text += i.path.encode("utf8")
        text += NULL
        text += int(i.hash, 16).to_bytes(20, byteorder ="big")
    return text






