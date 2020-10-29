FROM debian:buster
SHELL ["/bin/bash", "-c"]
USER root

# Install Environment
RUN apt-get update && apt-get upgrade -y && apt-get install sudo build-essential curl -y
RUN apt-get update -y && apt-get install git -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Setup ENVs
ENV PATH="/root/.cargo/bin:${PATH}"

# Configure
RUN git clone https://github.com/pseguin2011/dame_de_pique
RUN cd dame_de_pique/server && cargo build
RUN chmod -R 777 /dame_de_pique -R

# Running Image
EXPOSE 8000/tcp
CMD ["./dame_de_pique/server/target/debug/server"]