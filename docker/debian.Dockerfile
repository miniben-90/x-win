FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN echo "....install packages...." && \
  DEBIAN_FRONTEND=noninteractive && \
  apt-get update && \
  apt-get install --no-install-recommends -y \
  lsb-release \
  xvfb \
  locales \
  yaru-theme-icon \
  libproj15 \
  libatk1.0-0 \
  libatkmm-1.6-1v5 \
  libcairo2 \
  libgdk-pixbuf2.0-0 \
  libgl1 \
  libglib2.0-0 \
  libglibmm-2.4-1v5 \
  libgssapi-krb5-2 \
  libgtk-3-0 \
  libgtk2.0-0 \
  libgtkmm-3.0-1v5 \
  libmysqlclient21 \
  libpango-1.0-0 \
  libpangocairo-1.0-0 \
  libpcrecpp0v5 \
  libpng16-16 \
  libpython2.7 \
  libsecret-1-0 \
  libsigc++-2.0-0v5 \
  libsqlite3-0 \
  libssh-4 \
  libssl1.1 \
  libx11-6 \
  libxml2 \
  libzip5 \
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