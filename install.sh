#!/bin/sh
set -e

INSTALL_DIR=${INSTALL_DIR:-"/usr/local/bin"}
BINARY_NAME=${BINARY_NAME:-"sideko"}

REPO_NAME="sideko-inc/sideko"
ISSUE_URL="https://github.com/Sideko-Inc/sideko/issues/new"

get_latest_release() {
  curl --retry 5 --silent "https://api.github.com/repos/$1/releases/latest" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

get_asset_name() {
  os=$(get_os)
  if [[ "$os" == "windows" ]]; then
    echo "sideko-$1-$2.zip"
  else
    echo "sideko-$1-$2.tar.gz"
  fi
}

get_download_url() {
  local asset_name=$(get_asset_name $2 $3)
  echo "https://github.com/Sideko-Inc/sideko/releases/download/v$1/${asset_name}"
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
  echo "\`$(printf '\033[38;5;247m')$@${RESET}\`"
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

get_os() {
  case "$(uname -s)" in
    *linux* | *Linux* ) 
      echo "linux" 
      ;;
    *darwin* | *Darwin* ) 
      echo "darwin" 
      ;;
    CYGWIN* | MINGW* | MSYS* ) 
      echo "windows" 
      ;;
    *)
      echo "unknown"
      ;;
  esac
}
get_machine() {
  case "$(uname -m)" in
    "x86_64"|"amd64"|"x64")
      echo "x86_64" ;;
    "i386"|"i86pc"|"x86"|"i686")
      echo "386" ;;
    "arm64"|"armv6l"|"aarch64")
      echo "aarch64"
  esac
}

get_tmp_dir() {
  echo $(mktemp -d)
}


do_install_binary() {
  asset_name=$(get_asset_name $os $machine)
  download_url=$(get_download_url $version $os $machine)

  command_exists curl || {
    fmt_error "curl is not installed"
    exit 1
  }

  local tmp_dir=$(get_tmp_dir)

  # Download tar.gz to tmp directory
  echo "Downloading $download_url"
  (cd $tmp_dir && curl -sL -O "$download_url")

  cd $tmp_dir

  # Extract download
  if [[ "$asset_name" == *.tar.gz ]]; then
    tar -xvf "$asset_name"
  elif [[ "$asset_name" == *.zip ]]; then
    command_exists unzip || {
        fmt_error "unzip is not installed"
        exit 1
    }
    unzip "$asset_name"
  else
    fmt_error "Unknown file format: $asset_name"
    exit 1
  fi

  # Install binary
  sudo_cmd='mv '"$tmp_dir/$BINARY_NAME"' '"$INSTALL_DIR"' && chmod a+x '"$INSTALL_DIR/$BINARY_NAME"
  sudo -p "sudo password required for installing to $INSTALL_DIR: " -- sh -c "$sudo_cmd"
  echo "Installed sideko to $INSTALL_DIR"

  # Cleanup
  rm -rf $tmp_dir
}

main() {
  setup_color

  latest_tag=$(get_latest_release $REPO_NAME)
  latest_version=$(echo $latest_tag | sed 's/v//')
  version=${VERSION:-$latest_version}

  os=$(get_os)
  if test -z "$os"; then
    fmt_error "$(uname -s) os type is not supported"
    echo "Please create an issue so we can add support. $ISSUE_URL"
    exit 1
  fi

  machine=$(get_machine)
  if test -z "$machine"; then
    fmt_error "$(uname -m) machine type is not supported"
    echo "Please create an issue so we can add support. $ISSUE_URL"
    exit 1
  fi
  do_install_binary

  printf "$YELLOW"
  cat <<'EOF'

   _____ _     _      _             _____ _      _____  
  / ____(_)   | |    | |           / ____| |    |_   _| 
 | (___  _  __| | ___| | _____    | |    | |      | |   
  \___ \| |/ _` |/ _ \ |/ / _ \   | |    | |      | |   
  ____) | | (_| |  __/   < (_) |  | |____| |____ _| |_  
 |_____/|_|\__,_|\___|_|\_\___/    \_____|______|_____|

 The Sideko CLI is now Installed!
 Run `sideko --help` for help


EOF
  printf "$RESET"

}

main
