#!bin/bash

set -e

curl -L -o /tmp/firefox.tar.xz "https://download.mozilla.org/?product=firefox-latest&os=linux64"

sudo tar -xvf /tmp/firefox.tar.xz -C /opt/
