FROM busybox
# FROM rust:1.77

# ENV GO_VERSION='1.22.3'

# # ENV GETH_VERSION='1.14.3-ab48ba42'
# ENV GETH_VERSION='1.13.15-c5ba367e'
# RUN apt-get update \
#   && apt-get install -y bash curl gcc git libclang-dev \
#   && curl -LO https://gethstore.blob.core.windows.net/builds/geth-linux-amd64-${GETH_VERSION}.tar.gz \
#   && tar xvfz geth-linux-amd64-${GETH_VERSION}.tar.gz \
#   && mv geth-linux-amd64-${GETH_VERSION}/geth /usr/local/bin \
#   && rm -rf /usr/local/go \
#   && curl -LO https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz \
#   && tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz

# ENV PATH=${PATH}:/usr/local/go/bin

# WORKDIR /root

# ARG OP_VERSION="v1.7.4"
# ARG OP_GETH_VERSION="v1.101315.0"

# RUN git clone https://github.com/ethereum-optimism/optimism.git \
#   && cd optimism \
#   && git checkout ${OP_VERSION} \
#   && make op-node op-batcher op-proposer \
#   && cd .. \
#   && git clone https://github.com/ethereum-optimism/op-geth.git \
#   && cd op-geth \
#   && git checkout ${OP_GETH_VERSION} \
#   && make geth
