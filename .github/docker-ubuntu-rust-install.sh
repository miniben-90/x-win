#!bin/bash

curl https://sh.rustup.rs -sSf | bash -s -- -y

sudo apt install musl-tools -y

rustup target add $CARGO_TARGET_ENV

curl -L -o /tmp/firefox.tar.bz2 "https://download.mozilla.org/?product=firefox-latest&os=linux64"

sudo tar xjf /tmp/firefox.tar.bz2 -C /opt/

/opt/firefox/firefox --safe-mode https://github.com/ &

sleep 1

cd /work

cargo test --target $CARGO_TARGET_ENV
