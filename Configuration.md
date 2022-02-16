# Configuration

This is the default configuration. drop this in your `.bashrc` (or `.zshrc`) to get started.
remember to `source ~/.bashrc` (or `source ~/.zshrc`) to observe the changes!

```shell
# prompt string to display, for regular users
export IAY_PROMPT_CHAR="$"
export IAY_PROMPT_CHAR_COLOR="green"

# prompt string to display, for the root user
export IAY_PROMPT_CHAR_ROOT="#"
export IAY_PROMPT_CHAR_ROOT_COLOR="red"

# if EXPAND_TILDE is set to 0, `/home/aaqa` is shortened to `~`
export IAY_EXPAND_TILDE=0

# if SHORTEN_CWD is set to 1, `/home/aaqa/work` is shortened to
# `/h/a/work`
export IAY_SHORTEN_CWD=1

# Set colour for the directory
# - When nested inside the user's home directory
# - When not nested inside the user's home directory
export IAY_CWD_HOME_COLOR="white"
export IAY_CWD_ROOT_COLOR="blue"

# Disable git information
export IAY_DISABLE_VCS=1

# there are three possible states for a git repo
# - unstaged (working tree has been modified)
# - staged (staging area has been modified)
# - clean (all staged changes have committed)

# colour to represent clean repo state
export IAY_GIT_CLEAN_COLOR="green"

# symbol to represent unstaged repo state
export IAY_GIT_STATUS_STAGED="±"
export IAY_GIT_WT_MODIFIED="cyan"

# symbol to represent staged repo state
export IAY_GIT_STATUS_STAGED="±"
export IAY_GIT_INDEX_MODIFIED_COLOR="yellow"

# synbol to represent new file
export IAY_GIT_WT_ADDED_COLOR="!"
export IAY_GIT_WT_ADDED_COLOR="magenta"

# symbol to represent stashed changes indicator
export IAY_GIT_STATUS_STASH="$"

# Change branch colour when HEAD modifies
export IAY_GIT_BRANCH_MODIFIED_COLOR="blue"

# if HEAD ref peels to branch
export IAY_BRANCH_COLOR="green"

# if HEAD ref peels to a commit (detached state)
export IAY_COMMIT_COLOR="green"
```

All 16 colors are available:

```shell
black
red
green
yellow
blue
magenta (or purple)
cyan
white

bright black
bright red
bright green
bright yellow
bright blue
bright magenta (or purple)
bright cyan
bright white
```
