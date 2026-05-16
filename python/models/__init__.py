# models/__init__.py
from .object import GitObject, GitBlob
from .commit import GitCommit, GitTag
from .tree import GitTree
from .index import GitIndex, GitIndexEntry, GitIgnore

# This tells Python what to export when someone does "import *"
__all__ = ["GitObject", "GitBlob", "GitCommit", "GitTree","GitTag", "GitIndex", "GitIndexEntry", "GitIgnore"]