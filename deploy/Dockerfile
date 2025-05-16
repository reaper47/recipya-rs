# Stage 1: Build
FROM rust:1.86.0 AS build

# docker build . --build-arg="ARCH=x86_64" or docker build . --build-arg="ARCH=aarch64"
# docker-compose build --build-arg "ARCH=x86_64" or docker-compose build --build-arg "ARCH=aarch64"
ARG ARCH=x86_64

RUN apt-get update && \
    apt-get install -y --no-install-recommends ffmpeg pax-utils libpq5 && \
    mkdir -p /ffmpeg-deps/usr/bin /ffmpeg-deps/usr/lib/${ARCH}-linux-gnu /ffmpeg-deps/lib/${ARCH}-linux-gnu && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/recipya
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf src
COPY . .
COPY ./deploy/.env.example .env

RUN cp /usr/bin/ffmpeg /usr/bin/ffprobe /ffmpeg-deps/usr/bin/
RUN for bin in /usr/bin/ffmpeg /usr/bin/ffprobe; do \
      lddtree -l $bin | grep -v '^(' | xargs -I '{}' cp -v --parents '{}' /ffmpeg-deps/; \
    done

RUN cargo install --path . --locked

# ---------------------------------------------------
# 2 - Deploy Stage
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian12

ARG ARCH
ARG DEB_ARCH

COPY --from=build /ffmpeg-deps/usr /usr
COPY --from=build /ffmpeg-deps/lib /lib

# libpq related (required by diesel)
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libldap-2.5.so.* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/liblber-2.5.so.* /usr/lib/${ARCH}-linux-gnu/

# Application files
COPY --from=build /usr/local/cargo/bin/recipya-rs /usr/local/bin/recipya
COPY --from=build /usr/src/recipya/.env /.env

CMD ["/usr/local/bin/recipya", "server"]
