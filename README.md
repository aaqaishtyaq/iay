# iay

{ba, z}sh prompt written in rust.

```console
~/D/g/a/iay main [!] %
```

- bash users, set your `PS1`:

```shell
PS1='$(iay)'    # regular variant
PS1='$(iay -m)' # minimal variant
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

![iay prompt](https://github.com/dvit0/fsrv-cdn/raw/627d3f3697bb3fd9d104b90ebbbcb62fc11ea61c/files/iay_prompt.png)

## [Configure the prompt](Configuration.md)
