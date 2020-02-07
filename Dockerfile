# Build stage

FROM rust:latest AS build

WORKDIR /root
ADD . .

# We can't use the `rust:alpine` image directly,
# because we have proc macros crates in the dependency tree
# and they can't be compiled directly on musl systems.
# Cross compiling works, though, so here we are.
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target=x86_64-unknown-linux-musl --features vendored && \
    strip ./target/x86_64-unknown-linux-musl/release/unbound-telemetry

# Execution stage

FROM alpine:latest

RUN apk add --update curl && \
    rm -rf /var/cache/apk/*

COPY --from=build /root/target/x86_64-unknown-linux-musl/release/unbound-telemetry /bin

EXPOSE 80

HEALTHCHECK --timeout=1s CMD /usr/bin/curl --silent --fail http://127.0.0.1:80/healthcheck || exit 1

ENTRYPOINT ["/bin/unbound-telemetry"]
