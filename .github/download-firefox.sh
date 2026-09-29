#!/bin/bash

set -e

target_dist=$([ "$1" = "aarch64" ] && echo "-aarch64" || echo "")

echo "Start downloading firefox linux64${target_dist}"

curl -L -o /tmp/firefox.tar.xz "https://download.mozilla.org/?product=firefox-latest&os=linux64${target_dist}"
sudo tar -xvf /tmp/firefox.tar.xz -C /opt/
