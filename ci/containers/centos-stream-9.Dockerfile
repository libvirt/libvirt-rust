# THIS FILE WAS AUTO-GENERATED
#
#  $ lcitool manifest ci/manifest.yml
#
# https://gitlab.com/libvirt/libvirt-ci

FROM quay.io/centos/centos:stream9

RUN dnf distro-sync -y && \
    dnf install 'dnf-command(config-manager)' -y && \
    dnf config-manager --set-enabled -y crb && \
    dnf install -y \
        https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm \
        https://dl.fedoraproject.org/pub/epel/epel-next-release-latest-9.noarch.rpm && \
    dnf install -y \
        ca-certificates \
        cargo \
        clang-devel \
        clippy \
        gcc \
        git \
        glibc-langpack-en \
        libvirt-devel \
        pkgconfig \
        rust && \
    dnf autoremove -y && \
    dnf clean all -y && \
    rpm -qa | sort > /packages.txt

ENV LANG "en_US.UTF-8"
