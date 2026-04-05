from .object import GitObject

class GitCommit(GitObject):
    fmt = b'commit'

    def init(self):
        self.data = {}

    def dump(self):
        return write_metadata(self.data)
    
    def load(self,data):
        # raise ExceptionError
        # self.data = read_metadata(data)
        val = read_metadata(data)
        assert(type(val) == type({}))
        # print("-"*10)
        # print(val)
        # print("-"*10)
        self.data = val

class GitTag(GitCommit):
    fmt = b'tag'


def read_metadata(text):
    data = {}
    message_flag = 0
    data[None] = []
    SPACE = b' '
    NEWLINE = b'\n'

    start = 0
    end = 0

    while True:
        end = text.find(NEWLINE, start)
        if end < 0:
            break
        line = text[start:end]
        start = end + 1
        
            
        if message_flag:
            data[None].append(line)
            continue

        space_idx = line.find(SPACE)
        if space_idx < 0 :
            message_flag = 1
        elif space_idx == 0:
            data[key][-1] +=  line[space_idx+1:] + NEWLINE
        else:
            key = line[:space_idx]
            value = line[space_idx+1:] + NEWLINE
            if key not in data:
                data[key] = []
            data[key].append(value)
            
    data[None] = NEWLINE.join(data[None])
    return data

def write_metadata(data):
    text = b''
    SPACE = b' '
    NEWLINE = b'\n'

    for key in data:
        print(key)
        if key is None:
            continue
        val_list = data[key]
        if type(val_list) != list:
            val_list = [val_list]
        # else:
        for val in val_list:
            text += key + SPACE + val.replace(NEWLINE, NEWLINE + SPACE).rstrip(SPACE)
    text += NEWLINE + data[None]

    return text
