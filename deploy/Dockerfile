#FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
#WORKDIR /app

# --- Planner Stage: Cache dependencies ---
#FROM chef AS planner
#COPY ./web .
#COPY . .

#RUN rustup target add x86_64-unknown-linux-musl
#RUN cargo chef prepare --recipe-path recipe.json

# --- Builder Stage: Build the application ---
#FROM chef AS builder

#RUN curl -fsSL https://deb.nodesource.com/setup_23.x | bash - \
#	&& apt-get install -y nodejs build-essential pkg-config gcc-multilib libpq-dev
	
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json

#COPY ./web .
#COPY . .

#RUN cargo install cargo-metask
#RUN cargo task web-install-deps
#RUN cargo task web-build 

#RUN cargo build --release

# --- Release Stage: Final image ---
FROM alpine:latest AS release
LABEL org.opencontainers.image.source="https://github.com/reaper47/recipya-rs"
LABEL org.opencontainers.image.description="A clean, simple and powerful recipe manager your whole family will enjoy."
LABEL org.opencontainers.image.licenses="AGPL-3.0"

#WORKDIR /app

#RUN apk add --no-cache ffmpeg
#RUN adduser -D recipya
#USER recipya:recipya

#RUN mkdir -p /home/recipya/.config/Recipya

#COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/recipya .

COPY heavy-metal-notifier /usr/local/bin/recipya
COPY .env /app/.env

EXPOSE 8078
CMD ["/app/recipya", "serve"]
