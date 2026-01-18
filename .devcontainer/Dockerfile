FROM rust:1.91.1

WORKDIR /workspace

RUN rustup target add wasm32-unknown-unknown

RUN curl -sSL https://dioxus.dev/install.sh | bash -s v0.7.1

CMD ["bash"]
