# THIS FILE WAS AUTO-GENERATED
#
#  $ lcitool manifest ci/manifest.yml
#
# https://gitlab.com/libvirt/libvirt-ci

FROM docker.io/library/debian:12-slim

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y eatmydata && \
    eatmydata apt-get dist-upgrade -y && \
    eatmydata apt-get install --no-install-recommends -y \
                      ca-certificates \
                      cargo \
                      ccache \
                      git \
                      libclang-dev \
                      locales \
                      pkgconf \
                      rust-clippy \
                      rustc && \
    eatmydata apt-get autoremove -y && \
    eatmydata apt-get autoclean -y && \
    sed -Ei 's,^# (en_US\.UTF-8 .*)$,\1,' /etc/locale.gen && \
    dpkg-reconfigure locales

ENV CCACHE_WRAPPERSDIR "/usr/libexec/ccache-wrappers"
ENV LANG "en_US.UTF-8"

RUN export DEBIAN_FRONTEND=noninteractive && \
    dpkg --add-architecture armel && \
    eatmydata apt-get update && \
    eatmydata apt-get dist-upgrade -y && \
    eatmydata apt-get install --no-install-recommends -y dpkg-dev && \
    eatmydata apt-get install --no-install-recommends -y \
                      gcc-arm-linux-gnueabi \
                      libstd-rust-dev:armel \
                      libvirt-dev:armel && \
    eatmydata apt-get autoremove -y && \
    eatmydata apt-get autoclean -y && \
    mkdir -p /usr/local/share/meson/cross && \
    printf "[binaries]\n\
c = '/usr/bin/arm-linux-gnueabi-gcc'\n\
ar = '/usr/bin/arm-linux-gnueabi-gcc-ar'\n\
strip = '/usr/bin/arm-linux-gnueabi-strip'\n\
pkgconfig = '/usr/bin/arm-linux-gnueabi-pkg-config'\n\
\n\
[host_machine]\n\
system = 'linux'\n\
cpu_family = 'arm'\n\
cpu = 'arm'\n\
endian = 'little'\n" > /usr/local/share/meson/cross/arm-linux-gnueabi && \
    dpkg-query --showformat '${Package}_${Version}_${Architecture}\n' --show > /packages.txt && \
    mkdir -p /usr/libexec/ccache-wrappers && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/arm-linux-gnueabi-cc && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/arm-linux-gnueabi-gcc

ENV ABI "arm-linux-gnueabi"
ENV RUST_TARGET "armv5te-unknown-linux-gnueabi"
