# iay

{ba, z}sh prompt written in rust.

```console
~/D/g/a/iay main [!] %
```

- Bash users can set `PS1` directly:

```shell
PS1='$(iay)'    # regular variant
PS1='$(iay -m)' # minimal variant
```

  For a responsive prompt in large repositories, source the bundled asynchronous
  integration instead. It shows the last completed Git state immediately and
  computes the next one in the background:

```shell
source /path/to/iay/shell/iay.bash
```

- zsh users, add this to your `.zshrc`:

```shell
autoload -Uz add-zsh-hook
_iay_prompt() {
  PROMPT="$(iay -z)"    # regular variant
  # PROMPT="$(iay -zm)" # miminal variant
}
add-zsh-hook precmd _iay_prompt
```

  Or source the bundled asynchronous integration, which avoids blocking Zsh on
  Git status collection:

```shell
source /path/to/iay/shell/iay.zsh
```

  Set `IAY_COMMAND` before sourcing either file when `iay` is not on `PATH`.

## Verify prompt states

Run the disposable Git-state verifier to display and check clean, untracked,
unstaged, staged, stash, ahead, and behind prompts:

```shell
./scripts/verify-prompt
```

Set `IAY_COMMAND` to test an already-built binary instead of building the
current checkout.

## [Configure the prompt](Configuration.md)

Configurations for the prompt.

## Screenshots

![base16-iay](https://user-images.githubusercontent.com/22131756/204612606-4a130ea2-a940-4250-80b4-60cf6c0ccbeb.png)
![gruvbox-iay](https://user-images.githubusercontent.com/22131756/204612665-eeac4fc2-6ec1-421c-8843-e6883ff90e1a.png)
