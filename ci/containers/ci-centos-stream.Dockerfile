# THIS FILE WAS AUTO-GENERATED
#
#  $ lcitool dockerfile centos-stream-8 libvirt+dist,libvirt-rust
#
# https://gitlab.com/libvirt/libvirt-ci/-/commit/6cd723b4affb2ee67e7d462dac480191c4b97598

FROM quay.io/centos/centos:stream8

RUN dnf update -y && \
    dnf install 'dnf-command(config-manager)' -y && \
    dnf config-manager --set-enabled -y powertools && \
    dnf install -y centos-release-advanced-virtualization && \
    dnf install -y epel-release && \
    dnf install -y \
        ca-certificates \
        cargo \
        ccache \
        gcc \
        git \
        glibc-langpack-en \
        libvirt-devel \
        rust && \
    dnf autoremove -y && \
    dnf clean all -y && \
    rpm -qa | sort > /packages.txt && \
    mkdir -p /usr/libexec/ccache-wrappers && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/cc && \
    ln -s /usr/bin/ccache /usr/libexec/ccache-wrappers/gcc

ENV LANG "en_US.UTF-8"
ENV CCACHE_WRAPPERSDIR "/usr/libexec/ccache-wrappers"
