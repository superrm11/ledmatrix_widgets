#!/bin/sh

mkdir -p packages
docker build . -t ledmw_deb:latest
docker run --name ledmw_deb_latest ledmw_deb:latest
docker cp ledmw_deb_latest:/output/. ./packages/