FROM debian:bookworm-slim

WORKDIR /qcoin

COPY target/release/solochain-template-node /usr/local/bin/solochain-template-node
COPY qcoin_mainnet_spec.json /qcoin/qcoin_mainnet_spec.json

RUN chmod +x /usr/local/bin/solochain-template-node

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/solochain-template-node"]




