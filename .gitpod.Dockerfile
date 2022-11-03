FROM gitpod/workspace-full

USER gitpod

RUN rustup install nightly && \
    rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu &&\
    cargo install cargo-expand

# Install custom tools, runtime, etc. using apt-get
# For example, the command below would install "bastet" - a command line tetris clone:
#
# RUN sudo apt-get -q update && \
#     sudo apt-get install -yq bastet && \
#     sudo rm -rf /var/lib/apt/lists/*
#
# More information: https://www.gitpod.io/docs/config-docker/
