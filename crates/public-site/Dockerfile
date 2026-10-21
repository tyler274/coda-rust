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
# copied into the build, and only the public-site package is compiled.
COPY crates/ crates/
# Build these with the pinned Nix wasm-bindgen tool before invoking Docker (see
# book/src/public-site.md). Refuse a silently motion-less release image.
RUN test -s crates/public-site/island/dist/bootstrap.js \
    && test -s crates/public-site/island/dist/public_site_motion.js \
    && test -s crates/public-site/island/dist/public_site_motion_bg.wasm \
    && cargo build --locked --release -p public-site --bin public-site \
    && readelf -lW target/x86_64-unknown-linux-musl/release/public-site > /tmp/site-elf \
    && ! grep -q INTERP /tmp/site-elf

# Same pinned release/checksum as `workspace-tools install-doppler`. No unpinned
# package installation is required in either the builder or runtime image.
RUN wget -O /tmp/doppler.tar 'https://cli.doppler.com/download?os=linux&arch=amd64&format=tar&version=v3.77.0' \
    && printf '%s  %s\n' '92587cb89c2ff82fc2ddec2b2a9f275ff320994a44e16cd9310b55a429a82084' /tmp/doppler.tar | sha256sum -c - \
    && tar -xf /tmp/doppler.tar -C /tmp doppler \
    && chmod 0555 /tmp/doppler \
    && readelf -lW /tmp/doppler > /tmp/doppler-elf \
    && ! grep -q INTERP /tmp/doppler-elf

# distroless/static-debian12:nonroot
FROM ${RUNTIME_REPO}@sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab
COPY --from=build --chown=65532:65532 /src/target/x86_64-unknown-linux-musl/release/public-site /public-site
COPY --from=build --chown=65532:65532 /tmp/doppler /doppler
ENV PUBLIC_SITE_LISTEN_ADDR=0.0.0.0:8080
USER 65532:65532
EXPOSE 8080
# Local smoke tests need no Doppler account. Cloud Run explicitly overrides the
# command to /doppler run ... -- /public-site (see infra/gcp/public-site).
ENTRYPOINT ["/public-site"]
