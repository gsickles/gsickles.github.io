# Aliases
alias ll='ls -la'
HISTFILE=~/.history
HISTSIZE=32767
SAVEHIST=32767
setopt SHARE_HISTORY

autoload -U colors && colors
export PS1="%F{magenta}(%D{%Y-%m-%d %H:%M:%S.%.})%f [%3d] %F{yellow}%n%f@%F{cyan}%m %F{yellow}%f$ "
