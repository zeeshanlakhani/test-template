# syntax=docker/dockerfile:1

# AMD64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:x86_64-musl as builder-amd64

# ARM64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl as builder-arm64

ARG TARGETARCH
FROM builder-$TARGETARCH as builder

RUN adduser --disabled-password --disabled-login --gecos "" --no-create-home test-template

RUN cargo init

# touch lib.rs as we combine both
RUN touch src/lib.rs

# copy cargo.*
COPY ../test-template/Cargo.lock ./Cargo.lock
COPY ../test-template/Cargo.toml ./Cargo.toml

# cache depencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,target=$CARGO_HOME/.git \
    --mount=type=cache,target=test-template/target,sharing=locked \
    cargo build --target $CARGO_BUILD_TARGET --release

# copy src
COPY ../test-template/src ./src

# final build for release
RUN rm ./target/$CARGO_BUILD_TARGET/release/deps/*test_template*
RUN --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,target=$CARGO_HOME/.git \
    --mount=type=cache,target=test-template/target,sharing=locked \
    cargo build --target $CARGO_BUILD_TARGET --bin test-template --release

RUN musl-strip ./target/$CARGO_BUILD_TARGET/release/test-template
RUN mv ./target/$CARGO_BUILD_TARGET/release/test-template /usr/local/bin

FROM scratch

ARG backtrace=0
ARG log_level=info

ENV RUST_BACKTRACE=${backtrace} \
    RUST_LOG=${log_level}

COPY --from=builder /usr/local/bin/test-template .
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER test-template:test-template

ENTRYPOINT ["./test-template"]
