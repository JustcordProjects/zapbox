FROM debian:latest

ARG ZAP_VERSION=0.2.0

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    tar \
    clang \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /opt/zap && \
    curl -sSL \
    "https://github.com/thezaplang/zap/releases/download/v${ZAP_VERSION}/zap-${ZAP_VERSION}-linux-x86_64.tar.gz" \
    | tar -xzC /opt/zap --strip-components=1

ENV PATH="/opt/zap:${PATH}"

RUN printf '#!/bin/sh\nfile="$1"; shift; exec "$@" < "$file"\n' > /usr/local/bin/pipe && \
    chmod +x /usr/local/bin/pipe

WORKDIR /workspace
