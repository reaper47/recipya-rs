# ---------------------------------------------------
# 1 - Build Stage
# ---------------------------------------------------
FROM rust:1.87.0-slim AS build

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
        perl-base && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/recipya

ENV OPENSSL_NO_VENDOR=1

COPY Cargo.toml Cargo.lock ./

COPY crates/app/Cargo.toml crates/app/
COPY crates/auth/Cargo.toml crates/auth/
COPY crates/config/Cargo.toml crates/config/
COPY crates/email/Cargo.toml crates/email/
COPY crates/integrations/Cargo.toml crates/integrations/
COPY crates/models/Cargo.toml crates/models/
COPY crates/recipe-schema/Cargo.toml crates/recipe-schema/
COPY crates/recipya-cli/Cargo.toml crates/recipya-cli/
COPY crates/recipya-scraper/Cargo.toml crates/recipya-scraper/
COPY crates/repository/Cargo.toml crates/repository/
COPY crates/router/Cargo.toml crates/router/
COPY crates/support/Cargo.toml crates/support/
COPY crates/templates/Cargo.toml crates/templates/
COPY crates/testing/Cargo.toml crates/testing/

RUN mkdir -p crates/recipya-cli/src && \
    echo "fn main() {}" > crates/recipya-cli/src/main.rs && \
    cargo build -r --bin recipya-cli && \
    rm -rf crates/*/src target/release/deps/recipya*

COPY . .

RUN cargo build -r --locked

RUN mkdir -p /runtime-deps/usr/bin /runtime-deps/usr/lib /runtime-deps/lib

RUN cp /usr/bin/ffmpeg /usr/bin/ffprobe /runtime-deps/usr/bin/ && \
    lddtree -l /usr/bin/ffmpeg /usr/bin/ffprobe | \
    grep -v '^(' | \
    xargs -I '{}' cp -L --parents '{}' /runtime-deps/ 2>/dev/null || true

RUN find /usr/lib/${ARCH}-linux-gnu /lib/${ARCH}-linux-gnu -name "libpq.so*" -o \
         -name "libssl.so*" -o -name "libcrypto.so*" -o -name "libgssapi_krb5.so*" -o \
         -name "libldap*.so*" -o -name "libkrb5*.so*" -o -name "libk5crypto.so*" -o \
         -name "libsasl2.so*" -o -name "libgnutls.so*" -o -name "libcom_err.so*" -o \
         -name "libkeyutils.so*" | \
    xargs -I '{}' cp -L '{}' /runtime-deps/usr/lib/ 2>/dev/null || true

# ---------------------------------------------------
# 2 - Deploy Stage
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian12

LABEL org.opencontainers.image.source="https://github.com/reaper47/recipya-rs" \
      org.opencontainers.image.description="A clean, simple and powerful recipe manager your whole family will enjoy." \
      org.opencontainers.image.licenses="AGPL-3.0"

# Copy runtime dependencies
COPY --from=build /runtime-deps/usr /usr
COPY --from=build /runtime-deps/lib /lib

# Copy application files
COPY --from=build /usr/src/recipya/target/release/recipya-rs /usr/local/bin/recipya

USER nonroot:nonroot
EXPOSE 8078

CMD ["/usr/local/bin/recipya", "server"]
