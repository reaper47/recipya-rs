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
    && apt-get install -y --no-install-recommends postgresql-client \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="$NVM_DIR/versions/node/v${NODE_VERSION}/bin:$PATH"

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /workspace
