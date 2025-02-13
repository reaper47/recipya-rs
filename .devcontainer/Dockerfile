FROM mcr.microsoft.com/devcontainers/rust:latest

RUN curl -fsSL https://deb.nodesource.com/setup_23.x | bash - \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        postgresql-client \
        nodejs \
    && npm install -g npm@latest \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli
