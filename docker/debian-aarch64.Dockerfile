FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN apt-get update && \
  apt-get install -y --fix-missing \
  libxcb-randr0-dev \
  libxcb-xtest0-dev \
  libxcb-xinerama0-dev \
  libxcb-shape0-dev \
  libxcb-xkb-dev \
  libxcb-ewmh-dev \
  libxcb1-dev \
  libxcb-composite0-dev \
  libxcb-render0-dev \
  libxcb-xfixes0-dev \
  libx11-dev