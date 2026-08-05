# Stage 1: Build stage
FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    lld \
    cmake \
    make \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src

WORKDIR /qcoin
COPY . .

RUN cargo build --release

# Stage 2: Minimal runtime stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /qcoin

COPY --from=builder /qcoin/target/release/solochain-template-node /usr/local/bin/solochain-template-node
COPY qcoin_mainnet_spec.json /qcoin/qcoin_mainnet_spec.json

RUN chmod +x /usr/local/bin/solochain-template-node

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/solochain-template-node"]





