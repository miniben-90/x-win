#!bin/bash

node_version=${1:-20}

# Install nodejs version x.x
curl -fsSL https://deb.nodesource.com/setup_$node_version.x | sudo -E bash

sudo apt update

sudo apt upgrade -y

sudo apt install -y nodejs libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev pkg-config build-essential

# Install yarn glboal
npm install --global yarn

curl -L -o /tmp/firefox.tar.bz2 "https://download.mozilla.org/?product=firefox-latest&os=linux64"

sudo tar xjf /tmp/firefox.tar.bz2 -C /opt/

/opt/firefox/firefox --safe-mode https://github.com/ &

sleep 1
