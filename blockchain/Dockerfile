FROM ubuntu:latest AS base


RUN apt-get update && apt-get install -y \
    wget \
    gnupg \
    unzip \
    build-essential \
    git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

FROM base AS builder

ENV BITCOIN_VERSION 27.0

# Download and verify Bitcoin Core
RUN wget https://bitcoincore.org/bin/bitcoin-core-${BITCOIN_VERSION}/bitcoin-${BITCOIN_VERSION}-x86_64-linux-gnu.tar.gz \
    && wget https://bitcoincore.org/bin/bitcoin-core-${BITCOIN_VERSION}/SHA256SUMS \
    && wget https://bitcoincore.org/bin/bitcoin-core-${BITCOIN_VERSION}/SHA256SUMS.asc \
    && sha256sum --ignore-missing --check SHA256SUMS \
    && git clone https://github.com/bitcoin-core/guix.sigs \
    && gpg --import guix.sigs/builder-keys/* \
    && gpg --verify SHA256SUMS.asc \
    && tar -xvf bitcoin-${BITCOIN_VERSION}-x86_64-linux-gnu.tar.gz -C /usr/local --strip-components=1 \
    && rm -rf bitcoin-${BITCOIN_VERSION}-x86_64-linux-gnu.tar.gz SHA256SUMS SHA256SUMS.asc guix.sigs


FROM ubuntu:latest AS final

COPY --from=builder /usr/local /usr/local
COPY conf/bitcoin.conf /root/.bitcoin/bitcoin.conf

# Expose the default Bitcoin Core Ports
EXPOSE 8332 8333 18332 18333 18443 18444

# Set command to run bitcoin daemon
CMD ["bitcoind"]