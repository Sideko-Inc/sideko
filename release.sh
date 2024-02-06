#!/bin/bash

GREEN='\033[0;32m'  # GREEN
RED='\033[0;31m'    # RED
CYAN='\033[1;36m'   # CYAN BOLD
NC="\033[0m"        # NO COLOR

die () {
    printf >&2 "${RED}\n$@\n${NC}"
    exit 1
}

prefix=""
crate="core"

if [[ "$1" == "py" ]]; then
    prefix="py-"
    crate="sideko-py"
    shift
fi


# Get the version number from Cargo.toml
version=$(grep -m1 -o 'version = "[0-9]\+\.[0-9]\+\.[0-9]\+"' ${crate}/Cargo.toml | cut -d '"' -f 2)

# Get the latest git commit hash
git_hash=$(git rev-parse HEAD)

# Create the tag using the version and git hash
tag_name="${prefix}v${version}"
tag_message="${prefix}Sideko Version ${version}"

printf "\n${CYAN}Tag Name:${NC} ${GREEN}$tag_name${NC}\n\n"
 
printf "${CYAN}Most Recent Git Hash:${NC} ${GREEN}${git_hash}${NC}\n\n"

printf "${CYAN}Tag message:${NC} ${GREEN}${tag_message}${NC}\n\n"

printf "${CYAN}Generated these git commands:${NC}\n"
printf "git tag -a ${tag_name} ${git_hash} -m ${tag_message}\n"
printf "git push origin ${tag_name}\n\n"

while true; do
    read -p "Would you like to create and push the tag? (y/[n]) " yn
    case $yn in
        [Yy]* ) git tag -a "${tag_name}" "${git_hash}" -m "${tag_message}"; git push origin "${tag_name}"; break;;
        [Nn]* ) die "not tagging...";;
        "" ) die "not tagging..." ;;
        * ) echo "Please answer yes or no.";;
    esac
done