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

## [Configure the prompt](Configuration.md)

Configurations for the prompt.

## Screenshots

![base16-iay](https://user-images.githubusercontent.com/22131756/204612606-4a130ea2-a940-4250-80b4-60cf6c0ccbeb.png)
![gruvbox-iay](https://user-images.githubusercontent.com/22131756/204612665-eeac4fc2-6ec1-421c-8843-e6883ff90e1a.png)

