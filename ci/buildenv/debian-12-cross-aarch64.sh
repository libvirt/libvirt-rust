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
    dpkg --add-architecture arm64
    apt-get update
    apt-get dist-upgrade -y
    apt-get install --no-install-recommends -y dpkg-dev
    apt-get install --no-install-recommends -y \
            gcc-aarch64-linux-gnu \
            libstd-rust-dev:arm64 \
            libvirt-dev:arm64
    mkdir -p /usr/local/share/meson/cross
    printf "[binaries]\n\
c = '/usr/bin/aarch64-linux-gnu-gcc'\n\
ar = '/usr/bin/aarch64-linux-gnu-gcc-ar'\n\
strip = '/usr/bin/aarch64-linux-gnu-strip'\n\
pkgconfig = '/usr/bin/aarch64-linux-gnu-pkg-config'\n\
\n\
[host_machine]\n\
system = 'linux'\n\
cpu_family = 'aarch64'\n\
cpu = 'aarch64'\n\
endian = 'little'\n" > /usr/local/share/meson/cross/aarch64-linux-gnu
    dpkg-query --showformat '${Package}_${Version}_${Architecture}\n' --show > /packages.txt
    mkdir -p /usr/libexec/ccache-wrappers
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/aarch64-linux-gnu-cc
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/aarch64-linux-gnu-gcc
}

export CCACHE_WRAPPERSDIR="/usr/libexec/ccache-wrappers"
export LANG="en_US.UTF-8"

export ABI="aarch64-linux-gnu"
export RUST_TARGET="aarch64-unknown-linux-gnu"
