# THIS FILE WAS AUTO-GENERATED
#
#  $ lcitool manifest ci/manifest.yml
#
# https://gitlab.com/libvirt/libvirt-ci

FROM docker.io/library/almalinux:9

RUN dnf --quiet update -y && \
    dnf --quiet install 'dnf-command(config-manager)' -y && \
    dnf --quiet config-manager --set-enabled -y crb && \
    dnf --quiet install -y epel-release && \
    dnf --quiet install almalinux-release-devel -y && \
    dnf --quiet config-manager --set-enabled -y devel && \
    dnf --quiet install -y \
                ca-certificates \
                cargo \
                ccache \
                clang-devel \
                clippy \
                gcc \
                git \
                glibc-langpack-en \
                libvirt-devel \
                pkgconfig \
                rust \
                rust-std-static && \
    dnf --quiet autoremove -y && \
    dnf --quiet clean all -y && \
    rpm -qa | sort > /packages.txt && \
    mkdir -p /usr/libexec/ccache-wrappers && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/cc && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/gcc

ENV CCACHE_WRAPPERSDIR="/usr/libexec/ccache-wrappers"
ENV LANG="en_US.UTF-8"
