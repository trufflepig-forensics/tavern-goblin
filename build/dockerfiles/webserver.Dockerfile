ARG RUST_VERSION=1.85.0

FROM rust:${RUST_VERSION}-slim-bookworm AS buildrust

WORKDIR /app

RUN <<EOF
apt-get update
apt-get install openssl libssl-dev pkg-config mold -y
apt-get install protobuf-compiler -y
EOF

COPY ./build/webserver/cargo-config.toml .cargo/config.toml

RUN --mount=type=bind,source=webserver/,target=webserver/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release --locked
cp ./target/release/webserver /bin/server
EOF


FROM debian:bookworm-slim AS final

RUN <<EOF
apt-get update
apt-get install -y libssl-dev ca-certificates sudo
EOF

# Copy startup script
COPY ./build/webserver/startup.sh /
RUN chmod +x /startup.sh

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10000
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Allow appuser to execute update-ca-certificates
COPY ./build/webserver/sudoers-appuser /etc/sudoers.d/appuser

# Copy migrations
COPY ./webserver/migrations /migrations

# Copy the executable from the "build" stage.
COPY --from=buildrust /bin/server /bin/

USER appuser

# What the container should run when it is started.
CMD ["/startup.sh"]