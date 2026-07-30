

# iay

Prompt para {ba, z}sh escrito en Rust.

```console
~/D/g/a/iay main [!] %
```

- Los usuarios de Bash pueden configurar `PS1` directamente:

```shell
PS1='$(iay)'    # regular variant
PS1='$(iay -m)' # minimal variant
```

  Para tener un prompt responsivo en repositorios grandes, importa en su lugar la integración asíncrona incluida. Muestra inmediatamente el último estado de Git completado y calcula el siguiente en segundo plano:

```shell
source /path/to/iay/shell/iay.bash
```

- Los usuarios de zsh deben agregar esto a su `.zshrc`:

```shell
autoload -Uz add-zsh-hook
_iay_prompt() {
  PROMPT="$(iay -z)"    # regular variant
  # PROMPT="$(iay -zm)" # miminal variant
}
add-zsh-hook precmd _iay_prompt
```

  O importa la integración asíncrona incluida, lo cual evita bloquear Zsh durante la recopilación del estado de Git:

```shell
source /path/to/iay/shell/iay.zsh
```

  Establece `IAY_COMMAND` antes de importar cualquiera de los archivos cuando `iay` no se encuentre en `PATH`.

## Verificar los estados del prompt

Ejecuta el verificador de estado de Git desechable para mostrar y comprobar los prompts de limpio, sin seguimiento, sin añadir, añadido, guardado (stash), por delante y por detrás:

```shell
./scripts/verify-prompt
```

Establece `IAY_COMMAND` para probar un binario ya compilado en lugar de compilar el checkout actual.

## [Configurar el prompt](Configuration.md)

Configuraciones para el prompt.

## Capturas de pantalla

![base16-iay](https://user-images.githubusercontent.com/22131756/204612606-4a130ea2-a940-4250-80b4-60cf6c0ccbeb.png)
![gruvbox-iay](https://user-images.githubusercontent.com/22131756/204612665-eeac4fc2-6ec1-421c-8843-e6883ff90e1a.png)
