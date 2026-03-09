# ---------------------------------------------------
# 1. Build Stage
# ---------------------------------------------------
FROM rust:1.93 AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ffmpeg \
    pax-utils \
    ca-certificates \
    perl \
    perl-base \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/recipya
ENV CC=clang
ENV CXX=clang++
ENV OPENSSL_NO_VENDOR=1

COPY . .
RUN cargo build -r --locked
RUN cp /usr/src/recipya/target/release/recipya-cli /usr/local/bin/recipya-cli

# ---------------------------------------------------
# 2. Deploy Stage
# ---------------------------------------------------
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    libgssapi-krb5-2 \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

LABEL org.opencontainers.image.source="https://github.com/reaper47/recipya-rs" \
    org.opencontainers.image.description="A clean, simple and powerful recipe manager your whole family will enjoy." \
    org.opencontainers.image.licenses="AGPL-3.0"

COPY --from=builder /usr/local/bin/recipya-cli /usr/local/bin/recipya

RUN useradd -r -u 65532 -s /bin/false recipya && \
    mkdir -p /home/recipya/.local/share/Recipya && \
    chown -R recipya:recipya /home/recipya
USER recipya:recipya

EXPOSE 8078
CMD ["/usr/local/bin/recipya", "server"]
