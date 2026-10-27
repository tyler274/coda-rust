# Build from the workspace root with --platform=linux/amd64.
# Base images come from private Artifact Registry repositories in
# coda-health-staging, never straight from a public registry:
#   public-site-dockerhub  remote (pull-through) repository for Docker Hub
#   public-site-base       standard repository holding a digest-identical copy
#                          of gcr.io/distroless/static-debian12:nonroot
# Authenticate once: gcloud auth configure-docker us-east1-docker.pkg.dev
# The digests are fixed below; the ARGs choose only the registry path, so an
# override can relocate a base image but cannot change its content.
# Cargo.lock and --locked pin the Rust dependencies.
ARG RUST_REPO=us-east1-docker.pkg.dev/coda-health-staging/public-site-dockerhub/library/rust
ARG RUNTIME_REPO=us-east1-docker.pkg.dev/coda-health-staging/public-site-base/distroless/static-debian12

# rust:1.95.0-alpine3.23
FROM ${RUST_REPO}@sha256:606fd313a0f49743ee2a7bd49a0914bab7deedb12791f3a846a34a4711db7ed2 AS build
WORKDIR /src
ENV CARGO_INCREMENTAL=0 \
    CARGO_BUILD_TARGET=x86_64-unknown-linux-musl \
    RUSTFLAGS="-C target-feature=+crt-static -C strip=symbols --remap-path-prefix=/src=."
COPY Cargo.toml Cargo.lock ./
# Cargo resolves all workspace manifests; no reference submodules or secrets are
# copied into the build, and only the gmail package is compiled.
COPY crates/ crates/
RUN cargo build --locked --release -p gmail --bin mail-relay \
    && readelf -lW target/x86_64-unknown-linux-musl/release/mail-relay > /tmp/mail-elf \
    && ! grep -q INTERP /tmp/mail-elf

# distroless/static-debian12:nonroot
FROM ${RUNTIME_REPO}@sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab
COPY --from=build --chown=65532:65532 /src/target/x86_64-unknown-linux-musl/release/mail-relay /mail-relay
ENV MAIL_LISTEN_ADDR=0.0.0.0:8080
USER 65532:65532
EXPOSE 8080
ENTRYPOINT ["/mail-relay"]
