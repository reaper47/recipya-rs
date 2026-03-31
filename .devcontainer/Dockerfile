FROM mcr.microsoft.com/devcontainers/rust:latest

ENV NVM_DIR=/usr/local/nvm
ENV NODE_VERSION=23

RUN mkdir -p $NVM_DIR \
    && curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash \
    && . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm use $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && npm install -g npm@latest \
    && apt-get update \
    && apt-get install -y --no-install-recommends postgresql-client cmake clang \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="$NVM_DIR/versions/node/v${NODE_VERSION}/bin:$PATH"
ENV CC=clang
ENV CXX=clang++
ENV OPENSSL_NO_VENDOR=0
ENV PQ_LIB_STATIC=1

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /workspace
