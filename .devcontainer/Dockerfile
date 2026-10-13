# Dev-container image for the coda-rust workspace. Deliberately thin: the
# toolchain lives in the repository's nix dev shell, not here. The two extra
# packages (xz for the fallback Nix tarball, psql for the migration applier)
# install at container runtime in postCreate.sh, where compose's `dns:` fix
# applies; the host's systemd-resolved forwards to a DoT upstream that refuses
# plain DNS from container namespaces, and buildkit exposes no per-build DNS.
FROM mcr.microsoft.com/devcontainers/base@sha256:6ee2b4097c75d33e9a94d941577a3286e5d629434fba384688b06daeeaaeac99

# The host's NixOS daemon admits socket connections by effective group
# (allowed-users = @users, gid 100); the compose file runs the container user
# under that primary group. Make the passwd entry agree so login shells
# resolve gid 100 to `users`.
RUN groupadd -g 100 users || true \
  && usermod -g 100 vscode || true

# The nix dev shell needs flakes. The store daemon is absent inside a
# user-owned container, so build sandboxing and seccomp filtering are off.
COPY nix.conf /etc/nix/nix.conf
