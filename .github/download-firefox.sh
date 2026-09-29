#!bin/bash

set -e

if [[ "$1" == "aarch64"]]; then
  DIST="-aarch64"
else
  DIST=""
fi

echo "Start downloading firefox linux64${DIST}"
curl -L -o /tmp/firefox.tar.xz "https://download.mozilla.org/?product=firefox-latest&os=linux64${DIST}"
sudo tar -xvf /tmp/firefox.tar.xz -C /opt/
