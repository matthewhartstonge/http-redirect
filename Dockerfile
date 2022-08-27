################################################################################
#                                 Configure                                    #
################################################################################
ARG TARGET="x86_64-unknown-linux-musl"
ARG RUSTFLAGS="-C target-feature=+crt-static"
ARG BUILD_PATH="/rs/target/${TARGET}/release/https-redirect"

################################################################################
#                                  Compile                                     #
################################################################################
# Build our binary using the Rust SDK, taking in the required global docker
# arg defaults for this build scope.
#
# Currently we need a glibc based OS which supports dynamic linking in order to
# build a static binary. It will fail if we using alpine for this...
# Refer: https://github.com/rust-lang/rust/issues/40174#issuecomment-538791091
FROM rust:1.63-slim AS rustc
ARG TARGET
ARG RUSTFLAGS
ARG BUILD_ROOT
ENV RUSTFLAGS=${RUSTFLAGS}
RUN rustup target add ${TARGET}
WORKDIR /rs/
COPY . /rs/
RUN cargo build --target ${TARGET} --release

################################################################################
#                                Containerize                                  #
################################################################################
FROM scratch
ARG BUILD_PATH
COPY --from=rustc ${BUILD_PATH} /
USER 1000
CMD ["/https-redirect"]
