#!bin/bash

node_version=${1:-22}

# Install nodejs version x.x
curl -fsSL https://deb.nodesource.com/setup_$node_version.x | sudo -E bash

sudo apt update

sudo apt upgrade -y

sudo apt install -y nodejs libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev pkg-config build-essential

# Install yarn glboal
npm install --global yarn

sh /usr/local/download-firefox.sh

/opt/firefox/firefox --safe-mode https://github.com/ &

sleep 1
