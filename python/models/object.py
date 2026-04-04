from abc import ABC, abstractmethod

class GitObject(ABC):
    # def __init__(self, data = None):
    #     self.data = data

    def __init__(self, data=None):
        if data != None:
            self.load(data)
        else:
            self.init()

    @abstractmethod
    def dump(self):
        pass

    @abstractmethod
    def load(self,data):
        pass

class GitBlob(GitObject):
    fmt=b'blob'

    def dump(self):
        return self.data

    def load(self,data):
        self.data = data


