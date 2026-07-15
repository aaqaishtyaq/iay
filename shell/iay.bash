# Non-blocking Bash integration for iay.
# Source this after Bash's other prompt configuration.

_iay_async_dir=$(mktemp -d "${TMPDIR:-/tmp}/iay.XXXXXX") || return
_iay_async_result="$_iay_async_dir/prompt"
_iay_async_pid=""
_iay_prompt="$(IAY_DISABLE_VCS=1 command "${IAY_COMMAND:-iay}")"

_iay_async_poll() {
  if [[ -f $_iay_async_result ]]; then
    IFS= read -r -d '' _iay_prompt < "$_iay_async_result" || true
    rm -f -- "$_iay_async_result"
    _iay_async_pid=""
  elif [[ -n $_iay_async_pid ]] && ! kill -0 "$_iay_async_pid" 2>/dev/null; then
    _iay_async_pid=""
  fi
}

_iay_async_start() {
  [[ -n $_iay_async_pid ]] && return
  (
    command "${IAY_COMMAND:-iay}" > "$_iay_async_result"
  ) &
  _iay_async_pid=$!
}

_iay_precmd() {
  _iay_async_poll
  PS1=$_iay_prompt
  _iay_async_start
}

PROMPT_COMMAND="_iay_precmd${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
trap 'rm -rf -- "$_iay_async_dir"' EXIT
