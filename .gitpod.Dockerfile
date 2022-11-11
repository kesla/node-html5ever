FROM gitpod/workspace-full

USER gitpod

RUN rustup install nightly && \
    rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu &&\
    cargo install cargo-expand rust-script
