# THIS FILE WAS AUTO-GENERATED
#
#  $ lcitool manifest ci/manifest.yml
#
# https://gitlab.com/libvirt/libvirt-ci

function install_buildenv() {
    export DEBIAN_FRONTEND=noninteractive
    apt-get update
    apt-get dist-upgrade -y
    apt-get install --no-install-recommends -y \
            ca-certificates \
            cargo \
            ccache \
            git \
            libclang-dev \
            locales \
            pkgconf \
            rust-clippy \
            rustc
    sed -Ei 's,^# (en_US\.UTF-8 .*)$,\1,' /etc/locale.gen
    dpkg-reconfigure locales
    export DEBIAN_FRONTEND=noninteractive
    dpkg --add-architecture armel
    apt-get update
    apt-get dist-upgrade -y
    apt-get install --no-install-recommends -y dpkg-dev
    apt-get install --no-install-recommends -y \
            gcc-arm-linux-gnueabi \
            libstd-rust-dev:armel \
            libvirt-dev:armel
    mkdir -p /usr/local/share/meson/cross
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
endian = 'little'\n" > /usr/local/share/meson/cross/arm-linux-gnueabi
    dpkg-query --showformat '${Package}_${Version}_${Architecture}\n' --show > /packages.txt
    mkdir -p /usr/libexec/ccache-wrappers
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/arm-linux-gnueabi-cc
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/arm-linux-gnueabi-gcc
}

export CCACHE_WRAPPERSDIR="/usr/libexec/ccache-wrappers"
export LANG="en_US.UTF-8"

export ABI="arm-linux-gnueabi"
export RUST_TARGET="armv5te-unknown-linux-gnueabi"
