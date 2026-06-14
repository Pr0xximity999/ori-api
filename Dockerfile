FROM rust:1.96.0-slim AS build
RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev adduser gcc && \
    update-ca-certificates

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN adduser --disabled-password --gecos "" --shell "/sbin/nologin" --no-create-home --uid 10001 "ori-api"

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM rust:1.96-alpine3.21
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

USER ori-api:ori-api
COPY --from=build --chown=ori-api:ori-api ./target/x86_64-unknown-linux-musl/release/ori-api /app/ori-api
COPY --from=build ./src/batadase.db .

ENTRYPOINT [ "./app/ori-api" ]