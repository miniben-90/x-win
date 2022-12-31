FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN echo "....install packages...." && \
  DEBIAN_FRONTEND=noninteractive && \
  apt-get update && \
  apt-get install --no-install-recommends -y \
  xvfb \
  x11-utils \
  x11-xserver-utils \
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
  libx11-xcb-dev \
  libx11-dev \
  libxcb-util-dev \
  libxcb-util0-dev && \
  echo "....cleanup...." && \
  apt-get autoclean -y && \
  apt-get autoremove -y && \
  echo "....generate local en_US...." && \
  locale-gen en_US.UTF-8

#Add Xorg configuration to use dummy desktop
ADD ./docker/config/xorg.conf /usr/share/X11/xorg.conf.d/xorg.conf
ADD ./docker/config/xorg.conf /etc/X11/xorg.conf

#Set env display to :0
ENV DISPLAY=:0 \
LANGUAGE="en_US.UTF-8" \
LANG="en_US.UTF-8"