__frankenline() {
  setopt localoptions pipefail no_aliases 2> /dev/null
  eval "frankenline"
  echo -n "${(q)item} "
  local ret=$?
  echo
  return $ret
}

frankenline-widget() {
  LBUFFER="${LBUFFER}$(__frankenline)"
  local ret=$?
  zle reset-prompt
  return $ret
}
zle     -N   frankenline-widget
bindkey '^F' frankenline-widget
