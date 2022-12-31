FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN apt-get update && \
  apt-get install -y --fix-missing \
  libgl1-mesa-dev \
  libxcb-composite0-dev \
  libxcb-damage0-dev \
  libxcb-dpms0-dev \
  libxcb-dri2-0-dev \
  libxcb-dri3-dev \
  libxcb-glx0-dev \
  libxcb-present-dev \
  libxcb-randr0-dev \
  libxcb-record0-dev \
  libxcb-render0-dev \
  libxcb-res0-dev \
  libxcb-screensaver0-dev \
  libxcb-shape0-dev \
  libxcb-shm0-dev \
  libxcb-sync-dev \
  libxcb-xf86dri0-dev \
  libxcb-xfixes0-dev \
  libxcb-xinerama0-dev \
  libxcb-xkb-dev \
  libxcb-xtest0-dev \
  libxcb-xv0-dev \
  libxcb-xvmc0-dev \
  libx11-xcb-dev
  # libxcb-randr0-dev \
  # libxcb-xtest0-dev \
  # libxcb-xinerama0-dev \
  # libxcb-shape0-dev \
  # libxcb-xkb-dev \
  # libxcb-ewmh-dev \
  # libxcb1-dev \
  # libxcb-composite0-dev \
  # libxcb-render0-dev \
  # libxcb-xfixes0-dev \
  # libx11-dev