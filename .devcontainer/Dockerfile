FROM debian:buster
SHELL ["/bin/bash", "-c"]
USER root

# Install Environment
RUN apt-get update && apt-get upgrade -y && apt-get install sudo build-essential curl gnupg -y
RUN apt-get update -y && apt-get install -y \
    git \
    procps \
    nano

RUN curl -sL https://deb.nodesource.com/setup_14.x | sudo bash -
RUN apt -y install nodejs

RUN mkdir -p ~/src \
    && mkdir ~/.npm-global \
    && npm install expo-cli --global

#used by react native builder to set the ip address, other wise 
#will use the ip address of the docker container.
ENV REACT_NATIVE_PACKAGER_HOSTNAME="10.0.0.2"


#set the npm global location for dev user
ENV NPM_CONFIG_PREFIX="/root/.npm-global"


RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Setup ENVs
ENV PATH="/root/.cargo/bin:/root/.npm-global:/root/.npm-global/bin:${PATH}"
RUN rustup component add rust-analysis rls rust-src rust-std
# ENV RUST_SRC_PATH="/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library"
