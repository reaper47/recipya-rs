FROM mcr.microsoft.com/devcontainers/rust:latest

RUN rm -f /etc/apt/sources.list.d/yarn.list

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        postgresql-client libpq-dev cmake clang pkg-config libssl-dev ffmpeg \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && npm install -g npm@latest \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV CC=clang
ENV CXX=clang++
ENV OPENSSL_NO_VENDOR=0
ENV PQ_LIB_STATIC=1

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /workspace
