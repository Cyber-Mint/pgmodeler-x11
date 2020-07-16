# Step : Build image
FROM ubuntu:18.04 as BUILD

LABEL maintainer="Michael van Niekerk <mvniekerk@gmail.com>"

ARG PGMODELER_VERSION=v0.9.2-beta
ENV PGMODELER_VERSION=$PGMODELER_VERSION

RUN echo "${PGMODELER_VERSION}"
RUN apt-get update; apt-get install -y build-essential checkinstall qt5-default libpq-dev libxml2-dev qttools5-dev-tools clang qt5-image-formats-plugins libqt5svg5 libqt5svg5-dev curl
RUN mkdir /source
WORKDIR /source
RUN curl "https://codeload.github.com/pgmodeler/pgmodeler/tar.gz/${PGMODELER_VERSION}" -o pgmodeler-0.9.2.tar.gz
RUN ls /source/; md5sum /source/pgm*
WORKDIR /source
RUN tar -xvzf pgmod*; rm pgmodeler-0.9.2.tar.gz ; mv pgmo* pgmodeler
WORKDIR /source/pgmodeler
COPY linuxdeploy.sh ./
RUN chmod +x ./linuxdeploy.sh; ./linuxdeploy.sh

WORKDIR /source/pgmodeler/dist/
RUN mv pg* pgmodeler.tar.gz

# Step : Create Runnable pgmodeler
FROM ubuntu:18.04
COPY --from=BUILD /source/pgmodeler/dist/pgmodeler.tar.gz /opt/
WORKDIR /opt
RUN tar -xvzf pgmodeler.tar.gz; mv pgmodeler.tar.gz /pgmodeler.tar.gz; mv pgmodeler* pgmodeler; \
   apt-get update; apt-get install -y libxml2 libgl1 libpng16-16 libharfbuzz-gobject0 libqt5gui5 libpq5; apt-get clean; rm -rf /var/cache /pgmodeler.tar.gz
CMD /opt/pgmodeler/start-pgmodeler.sh
