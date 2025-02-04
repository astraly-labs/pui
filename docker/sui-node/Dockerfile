# Build application
FROM rust:1.81-bullseye AS builder

ARG TARGETARCH
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "/sui"

# Install build dependencies
RUN apt-get update && apt-get install -y cmake clang

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY sui-execution sui-execution
COPY external-crates external-crates

# Set up cross-compilation targets if needed
RUN rustup target add i686-unknown-linux-gnu

# Build for the appropriate architecture
RUN case ${TARGETARCH} in \
     "amd64") cargo build --profile ${PROFILE} --bin sui-node ;; \
     "386") cargo build --profile ${PROFILE} --target i686-unknown-linux-gnu --bin sui-node ;; \
     *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
     esac

# Production Image
FROM debian:bullseye-slim AS runtime

ARG TARGETARCH
ARG PROFILE=release

# Install architecture-specific dependencies
RUN apt-get update && apt-get install -y ca-certificates curl \
     && case ${TARGETARCH} in \
     "amd64") apt-get install -y libjemalloc-dev && \
     echo "/usr/lib/x86_64-linux-gnu/libjemalloc.so" > /etc/ld.preload ;; \
     "386") apt-get install -y libjemalloc-dev && \
     echo "/usr/lib/i386-linux-gnu/libjemalloc.so" > /etc/ld.preload ;; \
     esac

WORKDIR "/sui"

# Copy the binary based on architecture
COPY --from=builder /sui/target/${TARGETARCH}/release/sui-node /opt/sui/bin/sui-node
# Support legacy usages
COPY --from=builder /sui/target/${TARGETARCH}/release/sui-node /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION

LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
LABEL org.opencontainers.image.description="PUI Node"
LABEL org.opencontainers.image.licenses="Apache-2.0"

ENV PATH="/opt/sui/bin:${PATH}"

ENTRYPOINT ["/opt/sui/bin/sui-node"]