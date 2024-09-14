# Build the .deb for Ubuntu / debian systems
FROM ubuntu:latest AS deb

RUN apt update; yes |apt install cargo libudev-dev pkg-config
RUN cargo install cargo-deb

WORKDIR /tmp
COPY . .

RUN cargo build
RUN cargo deb
RUN mkdir -p /output
RUN mv /tmp/target/debian/*.deb /output/

# Build .rpm file for fedora systems
FROM fedora:latest AS rpm
RUN yum -y update; yum -y install systemd-devel rpm-build cargo

WORKDIR /tmp
COPY . .

RUN cargo install cargo-rpm
RUN cargo build
RUN cargo rpm build

RUN mkdir -p /output
RUN mv /tmp/target/release/rpmbuild/RPMS/x86_64/*.rpm /output/

# Build .pkg.tar.zst for Archlinux
FROM archlinux:base-devel AS pkg
RUN pacman --noconfirm -Sy cargo
RUN useradd -m builduser
WORKDIR /home/builduser
COPY . .
RUN chown -R builduser:builduser .
USER builduser

RUN cargo build
RUN makepkg

USER root

RUN mkdir -p /output
RUN mv /home/builduser/*.pkg.tar.zst /output/
COPY --from=deb /output/. /output/
COPY --from=rpm /output/. /output/

CMD /bin/bash