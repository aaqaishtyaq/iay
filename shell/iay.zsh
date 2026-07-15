# Non-blocking Zsh integration for iay.
# Source this after Zsh's other prompt configuration.

autoload -Uz add-zsh-hook
typeset -g _iay_async_dir="$(mktemp -d "${TMPDIR:-/tmp}/iay.XXXXXX")" || return
typeset -g _iay_async_result="$_iay_async_dir/prompt"
typeset -g _iay_async_pid=""
typeset -g PROMPT="$(IAY_DISABLE_VCS=1 command "${IAY_COMMAND:-iay}" -z)"

_iay_async_poll() {
  if [[ -f $_iay_async_result ]]; then
    PROMPT="$(<"$_iay_async_result")"
    rm -f -- "$_iay_async_result"
    _iay_async_pid=""
  elif [[ -n $_iay_async_pid ]] && ! kill -0 "$_iay_async_pid" 2>/dev/null; then
    _iay_async_pid=""
  fi
}

_iay_async_start() {
  setopt localoptions no_bgnice
  [[ -n $_iay_async_pid ]] && return
  (
    local result
    result=$(mktemp "$_iay_async_result.XXXXXX") || exit
    command "${IAY_COMMAND:-iay}" -z > "$result"
    [[ -d $_iay_async_dir ]] && mv -f -- "$result" "$_iay_async_result"
  ) &!
  _iay_async_pid=$!
}

_iay_precmd() {
  _iay_async_poll
  _iay_async_start
}

add-zsh-hook precmd _iay_precmd
TRAPEXIT() { rm -rf -- "$_iay_async_dir" }
