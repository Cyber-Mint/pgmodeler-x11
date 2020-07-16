#!/bin/bash

export PGMODELER_VERSION=0.9.3-beta
echo "Building for version ${PGMODELER_VERSION}"
docker build -t pgmodeler-docker-x11/run:$PGMODELER_VERSION --build-arg PGMODELER_VERSION="${PGMODELER_VERSION}" --file=`pwd`/docker/Dockerfile .
