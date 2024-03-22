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
    dpkg --add-architecture mips64el
    apt-get update
    apt-get dist-upgrade -y
    apt-get install --no-install-recommends -y dpkg-dev
    apt-get install --no-install-recommends -y \
            gcc-mips64el-linux-gnuabi64 \
            libstd-rust-dev:mips64el \
            libvirt-dev:mips64el
    mkdir -p /usr/local/share/meson/cross
    printf "[binaries]\n\
c = '/usr/bin/mips64el-linux-gnuabi64-gcc'\n\
ar = '/usr/bin/mips64el-linux-gnuabi64-gcc-ar'\n\
strip = '/usr/bin/mips64el-linux-gnuabi64-strip'\n\
pkgconfig = '/usr/bin/mips64el-linux-gnuabi64-pkg-config'\n\
\n\
[host_machine]\n\
system = 'linux'\n\
cpu_family = 'mips64'\n\
cpu = 'mips64el'\n\
endian = 'little'\n" > /usr/local/share/meson/cross/mips64el-linux-gnuabi64
    dpkg-query --showformat '${Package}_${Version}_${Architecture}\n' --show > /packages.txt
    mkdir -p /usr/libexec/ccache-wrappers
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/mips64el-linux-gnuabi64-cc
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/mips64el-linux-gnuabi64-gcc
}

export CCACHE_WRAPPERSDIR="/usr/libexec/ccache-wrappers"
export LANG="en_US.UTF-8"

export ABI="mips64el-linux-gnuabi64"
export RUST_TARGET="mips64el-unknown-linux-gnuabi64"
