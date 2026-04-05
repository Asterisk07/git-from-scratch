import sys

def cat(x):
    sys.stdout.buffer.write(x)


import logging
from contextlib import contextmanager

@contextmanager
def silent():
    """
    Temporarily disables logging up to the specified level.
    Defaults to DEBUG.
    """
    # Capture the current global disable level
    previous_level = logging.root.manager.disable
    
    # Disable logging up to the requested level
    logging.disable(logging.DEBUG)
    
    try:
        yield
    finally:
        # Restore the original logging level
        logging.disable(previous_level)