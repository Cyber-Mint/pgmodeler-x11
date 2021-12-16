# Step : Build image
FROM ubuntu:20.04 as BUILD

LABEL maintainer="Michael van Niekerk <mvniekerk@gmail.com>"

ARG PGMODELER_VERSION=v0.9.4-beta1
ENV PGMODELER_VERSION=$PGMODELER_VERSION

RUN echo "${PGMODELER_VERSION}"
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install apt-utils
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tzdata
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y curl clang checkinstall build-essential
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y qt5-default
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libpq-dev pkg-config
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libxml2-dev
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y qttools5-dev-tools
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y qt5-image-formats-plugins
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libqt5svg5
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libqt5svg5-dev 
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates curl software-properties-common unzip
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y qttools5-dev 

RUN mkdir /source
WORKDIR /source
RUN update-ca-certificates
RUN curl "https://codeload.github.com/pgmodeler/pgmodeler/tar.gz/refs/tags/${PGMODELER_VERSION}" -o pgmodeler-source.tar.gz
RUN ls /source/; md5sum /source/pgm*
WORKDIR /source
RUN tar -xvzf pgmod*; rm pgmodeler-source.tar.gz ; mv pgmo* pgmodeler
WORKDIR /source/pgmodeler
RUN curl https://codeload.github.com/pgmodeler/plugins/zip/refs/heads/master -o plugins.zip && unzip ./plugins.zip && rm plugins.zip && mv plugins-master plugins
COPY linuxdeploy.sh ./
COPY plugins.pro ./plugins/plugins.pro
RUN chmod +x ./linuxdeploy.sh; ./linuxdeploy.sh

WORKDIR /source/pgmodeler/dist/
RUN mv pg* pgmodeler.tar.gz

# Step : Create Runnable pgmodeler
FROM ubuntu:20.04
COPY --from=BUILD /source/pgmodeler/dist/pgmodeler.tar.gz /opt/
WORKDIR /opt
RUN mkdir -p /var/cache/apt/archives/partial; tar -xvzf pgmodeler.tar.gz; mv pgmodeler.tar.gz /pgmodeler.tar.gz; mv pgmodeler* pgmodeler; \
   apt-get update; apt-get install -y libxml2 libgl1 libpng16-16 libharfbuzz-gobject0 libqt5gui5 libpq5; apt-get clean; rm -rf /var/cache /pgmodeler.tar.gz
CMD /opt/pgmodeler/pgmodeler
