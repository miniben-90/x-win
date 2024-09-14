FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

WORKDIR /var/x-win

# RUN echo "deb [arch=arm64] http://ports.ubuntu.com/ jammy main" > /etc/apt/sources.list.d/arm64-ports.list && \
# apt-get update && \
# apt-get upgrade -y --fix-missing
# RUN apt-get install -y --fix-missing --no-install-recommends gcc make gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu aptitude
RUN apt-get install -y --fix-missing --no-install-recommends libx11-dev libx11-dev libxcb-ewmh-dev libxcb-randr0-dev librust-xcb-dev librust-xcb+xlib-xcb-dev
RUN apt-get install -y --fix-missing --no-install-recommends pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev

# RUN aptitude install -y librust-xcb-dev:arm64 librust-xcb+xlib-xcb-dev:arm64

#libx11-xcb-dev libxkbcommon-dev libxcb1-dev libxcb-ewmh-dev libxcb-randr0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

COPY . .

RUN yarn install && \
yarn build --target aarch64-unknown-linux-gnu && \
aarch64-unknown-linux-gnu-strip *.node

RUN echo "#!bin/bash\neval \"$@\"" > /var/bin/entrypoint

ENTRYPOINT [ "/var/bin/entrypoint" ]
