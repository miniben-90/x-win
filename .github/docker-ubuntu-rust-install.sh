#!bin/bash

target=${1:-''}

curl https://sh.rustup.rs -sSf | bash -s -- -y

. "$HOME/.cargo/env"

sudo apt update

sudo apt upgrade -y

sudo apt install musl-tools -y

sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev pkg-config build-essential -y

rustup target add $target

sh /usr/local/download-firefox.sh

/opt/firefox/firefox --safe-mode https://github.com/ &

sleep 1
