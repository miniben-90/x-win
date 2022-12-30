FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN apt-get update && \
  apt-get install -y --fix-missing --no-install-recommends \
  libxcb-ewmh-dev \
  libxcb-randr0-dev \
  libxcb1-dev \
  libxcb-composite0-dev \
  libx11-dev && \
  apt-get autoremove -y