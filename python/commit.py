# from object import GitObject
# from init import 
from init import repo_file, repo_find
from object import object_find, object_read
import logging
# try:
#     import graphviz
# except:
#     pass
logger = logging.getLogger(__name__)


# def read_metadata(text):
#     data = {}
#     message_flag = 0
#     data[None] = []
#     SPACE = b' '
#     NEWLINE = b'\n'

#     start = 0
#     end = 0

#     while True:
#         end = text.find(NEWLINE, start)
#         if end < 0:
#             break
#         line = text[start:end]
#         start = end + 1
        
            
#         if message_flag:
#             data[None].append(line)
#             continue

#         space_idx = line.find(SPACE)
#         if space_idx < 0 :
#             message_flag = 1
#         elif space_idx == 0:
#             data[key][-1] +=  line[space_idx+1:] + NEWLINE
#         else:
#             key = line[:space_idx]
#             value = line[space_idx+1:] + NEWLINE
#             if key not in data:
#                 data[key] = []
#             data[key].append(value)
            
#     data[None] = NEWLINE.join(data[None])
#     return data

# def write_metadata(data):
#     text = b''
#     SPACE = b' '
#     NEWLINE = b'\n'

#     for key in data:
#         if key is None:
#             continue
#         val_list = data[key]
#         for val in val_list:
#             text += key + SPACE + val.replace(NEWLINE, NEWLINE + SPACE).rstrip(SPACE)
#     text += NEWLINE + data[None]
#     return text

# class GitCommit(GitObject):
#     fmt = b'commit'

#     def init(self):
#         self.data = {}

#     def dump(self):
#         return write_metadata(self.data)
    
#     def load(self,data):
#         self.data = read_metadata(data)

LOG_GRAPH = ''

def log_graphviz(repo, sha, seen):
    global LOG_GRAPH
    # print
    # sha =sha.rstrip()
    assert(sha[-1] != b'\n')

    if sha in seen:
        return
    seen.add(sha)

    try:
        commit = object_read(repo, sha)
        message = (commit.data)[None].decode("utf8").strip()
    except:
        commit = None
        message = ''

    message = message.replace("\\", "\\\\")
    message = message.replace("\"", "\\\"")

    if "\n" in message: # Keep only the first line
        message = message[:message.index("\n")]

    print(f"  c_{sha} [label=\"{sha[0:7]}: {message}\"]")
    LOG_GRAPH += f"  c_{sha} [label=\"{sha[0:7]}: {message}\"]"
    if commit is None:
        return
    assert commit.fmt==b'commit'

    if not b'parent' in commit.data.keys():
        # Base case: the initial commit.
        return

    parents = commit.data[b'parent']

    # if type(parents) != list:
    #     parents = [ parents ]

    for p in parents:
        p = p.decode("ascii")
        print (f"  c_{sha} -> c_{p};")
        LOG_GRAPH +=  f"  c_{sha} -> c_{p};"
        logger.debug("Getting graph of : %s", p)
        # try:
        log_graphviz(repo, p, seen)
        # except:
        #     pass

    

def cmd_log(args):
    global LOG_GRAPH
    repo = repo_find()

    # print("digraph wyaglog{")
    # print("  node[shape=rect]")
    LOG_GRAPH += ("digraph wyaglog{")
    LOG_GRAPH += ("  node[shape=rect]")
    log_graphviz(repo, object_find(repo, args.commit), set())
    # This creates or overwrites 'log.dot' in your current directory
    with open("log2.dot", "w") as f:
        f.write(LOG_GRAPH)

    import graphviz
        # 1. Create a Source object from the string
    LOG_GRAPH += "}"
    src = graphviz.Source(LOG_GRAPH)

    # 2. Render and open (view=True opens the PDF in Windows/WSL)
    # This creates a file named 'log.pdf'
    src.render('log', format='pdf', view=True)
    # print("}")

def kvlm_parse(raw, start=0, dct=None):
    if not dct:
        dct = dict()
        # You CANNOT declare the argument as dct=dict() or all call to
        # the functions will endlessly grow the same dict.

    # This function is recursive: it reads a key/value pair, then call
    # itself back with the new position.  So we first need to know
    # where we are: at a keyword, or already in the messageQ

    # We search for the next space and the next newline.
    spc = raw.find(b' ', start)
    nl = raw.find(b'\n', start)

    # If space appears before newline, we have a keyword.  Otherwise,
    # it's the final message, which we just read to the end of the file.

    # Base case
    # =========
    # If newline appears first (or there's no space at all, in which
    # case find returns -1), we assume a blank line.  A blank line
    # means the remainder of the data is the message.  We store it in
    # the dictionary, with None as the key, and return.
    if (spc < 0) or (nl < spc):
        assert nl == start
        dct[None] = raw[start+1:]
        return dct

    # Recursive case
    # ==============
    # we read a key-value pair and recurse for the next.
    key = raw[start:spc]

    # Find the end of the value.  Continuation lines begin with a
    # space, so we loop until we find a "\n" not followed by a space.
    end = start
    while True:
        end = raw.find(b'\n', end+1)
        if raw[end+1] != ord(' '): break

    # Grab the value
    # Also, drop the leading space on continuation lines
    value = raw[spc+1:end].replace(b'\n ', b'\n')

    # Don't overwrite existing data contents
    if key in dct:
        if type(dct[key]) == list:
            dct[key].append(value)
        else:
            dct[key] = [ dct[key], value ]
    else:
        dct[key]=value

    return kvlm_parse(raw, start=end+1, dct=dct)