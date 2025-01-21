#!/bin/sh
set -e

INSTALL_DIR=${INSTALL_DIR:-"/usr/local/bin"}
BINARY_NAME=${BINARY_NAME:-"sideko"}

REPO_NAME="sideko-inc/sideko"
ISSUE_URL="https://github.com/Sideko-Inc/sideko/issues/new"

get_latest_release() {
  local repo_name=$1
  local url="https://api.github.com/repos/$repo_name/releases/latest"
  local res
  res=$(curl --retry 5 --silent --fail "$url" ) || {
    return 1
  }

  echo "$res" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}


command_exists() {
  command -v "$@" >/dev/null 2>&1
}

fmt_error() {
  echo ${RED}"Error: $@"${RESET} >&2
}

fmt_warning() {
  echo ${YELLOW}"Warning: $@"${RESET} >&2
}

fmt_underline() {
  echo "$(printf '\033[4m')$@$(printf '\033[24m')"
}

fmt_code() {
  echo "$(printf '\033[38;5;247m')$@${RESET}"
}

setup_color() {
  # Only use colors if connected to a terminal
  if [ -t 1 ]; then
    RED=$(printf '\033[31m')
    GREEN=$(printf '\033[32m')
    YELLOW=$(printf '\033[33m')
    BLUE=$(printf '\033[34m')
    MAGENTA=$(printf '\033[35m')
    BOLD=$(printf '\033[1m')
    RESET=$(printf '\033[m')
  else
    RED=""
    GREEN=""
    YELLOW=""
    BLUE=""
    MAGENTA=""
    BOLD=""
    RESET=""
  fi
}


main() {
  setup_color

  command_exists curl || {
    fmt_error "curl is not installed"
    exit 1
  }

  latest_tag=$(get_latest_release $REPO_NAME) || {
    fmt_error "unable to determine latest release to install"
    exit 1
  }
  installer_url="https://github.com/Sideko-Inc/sideko/releases/download/$latest_tag/sideko-installer.sh"

  installer=$(curl --proto '=https' --tlsv1.2 -LsSf $installer_url) || {
    fmt_error "failed retrieving installer for release $latest_tag"
    exit 1
  }
  installer_out=$(echo "$installer" | sh) || {
    fmt_error "failed running installer for release $latest_tag"
  }
  
  echo "$installer_out"

  check="${GREEN}✔$@${RESET}"
  help_cmd=$(fmt_code "sideko --help")
  cat <<EOF

.*....*......*.....*......*....*........*....*.....

..####...######..#####...######..##..##...####..
.##........##....##..##..##......##.##...##..##.
..####.....##....##..##..####....####....##..##.
.....##....##....##..##..##......##.##...##..##.
..####...######..#####...######..##..##...####..
................................................

$check The Sideko CLI is now installed!
 Run $help_cmd for help

*....*......*.....*......*.....*......*.....*.....*

EOF
  printf "$RESET"

}

main