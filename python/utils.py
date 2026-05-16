import sys

def cat(x):
    sys.stdout.buffer.write(x)

import logging
from contextlib import contextmanager
import configparser

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

def gitconfig_read():
    xdg_config_home = os.environ["XDG_CONFIG_HOME"] if "XDG_CONFIG_HOME" in os.environ else "~/.config"
    configfiles = [
        os.path.expanduser(os.path.join(xdg_config_home, "git/config")),
        os.path.expanduser("~/.gitconfig")
    ]

    config = configparser.ConfigParser()
    config.read(configfiles)
    return config

def gitconfig_user_get(config):
    if "user" in config:
        if "name" in config["user"] and "email" in config["user"]:
            return f"{config['user']['name']} <{config['user']['email']}>"
    return None