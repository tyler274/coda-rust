# Build from the workspace root with --platform=linux/amd64.
# Both base images are content-addressed; Cargo.lock and --locked pin Rust deps.
FROM rust:1.95.0-alpine3.23@sha256:606fd313a0f49743ee2a7bd49a0914bab7deedb12791f3a846a34a4711db7ed2 AS build
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

# Same pinned release/checksum as scripts/gcp/install_doppler.sh. No unpinned
# package installation is required in either the builder or runtime image.
RUN wget -O /tmp/doppler.tar 'https://cli.doppler.com/download?os=linux&arch=amd64&format=tar&version=v3.77.0' \
    && printf '%s  %s\n' '92587cb89c2ff82fc2ddec2b2a9f275ff320994a44e16cd9310b55a429a82084' /tmp/doppler.tar | sha256sum -c - \
    && tar -xf /tmp/doppler.tar -C /tmp doppler \
    && chmod 0555 /tmp/doppler \
    && readelf -lW /tmp/doppler > /tmp/doppler-elf \
    && ! grep -q INTERP /tmp/doppler-elf

FROM gcr.io/distroless/static-debian12:nonroot@sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab
COPY --from=build --chown=65532:65532 /src/target/x86_64-unknown-linux-musl/release/public-site /public-site
COPY --from=build --chown=65532:65532 /tmp/doppler /doppler
ENV PUBLIC_SITE_LISTEN_ADDR=0.0.0.0:8080
USER 65532:65532
EXPOSE 8080
# Local smoke tests need no Doppler account. Cloud Run explicitly overrides the
# command to /doppler run ... -- /public-site (see infra/gcp/public-site).
ENTRYPOINT ["/public-site"]
