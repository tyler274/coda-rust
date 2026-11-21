# coda-sandbox

The Coda agent sandbox: a hardened NixOS container where coding agents (and you, over Remote-SSH) work on `coda-rust`, and disposable Firecracker microVMs that run its CI. The crate is a member of the root Cargo workspace; this directory is also its own flake, apart from the product flake, which builds only this crate from the root manifest and lockfile, so a host deploys it without the product flake's toolchain or other crates. It provides:

- `packages.coda-sandbox`: the `coda-sandbox` binary ([src](src)), with `doctor` and `status`.
- `nixosModules.default`: `services.coda-sandbox` ([nix/default.nix](nix/default.nix)), which declares the container, its token broker, the CI and `coda-carry`, and writes `/etc/coda-sandbox/config.toml` for the binary.
- `checks`: the binary's tests, Clippy and rustfmt; the `coda-carry` test; and two VM tests, `agent-sandbox` and `coda-firecracker`, that need KVM.

## Using the module

Add the flake as an input with `inputs.nixpkgs.follows = "nixpkgs"`, import `inputs.coda-sandbox.nixosModules.default`, and set the host's values:

```nix
services.coda-sandbox = {
  enable = true;
  user = "alice";                           # owns the host checkout coda-carry bundles
  requires = [ "agents-pool.service" ];     # units that make paths.root ready
  authorizedKeys = [ "ssh-ed25519 AAAA... coda-agent" ];
  memoryHigh = "20G";
  ci.enable = true;
};
```

| Option | Default | Meaning |
| --- | --- | --- |
| `enable` | `false` | The container, the CI spool, `coda-carry` and the config file |
| `user` | none | The host user who works with the sandbox |
| `requires` | `[ ]` | Units the container, spool and CI need; a missing disk fails only them |
| `uidBase` | `655360` | First host UID of the container's range; the agent is `uidBase + 1000` |
| `hostAddress`, `localAddress` | `10.233.1.1`, `10.233.1.2` | The container's veth pair |
| `memoryMax`, `memoryHigh` | `24G`, `memoryMax` | The container unit's memory limits |
| `authorizedKeys` | `[ ]` | Keys that may log in as `agent`; sshd runs only with one |
| `container.enable` | `true` | Off leaves the spool, CI and `coda-carry` |
| `container.name`, `container.sshHost` | `coda-agent` | The container's name and the SSH `Host` alias |
| `container.config` | `{ }` | Extra NixOS configuration inside the container |
| `paths.root` | `/agents/coda` | Holds `src`, `build` and `ci`; each is its own option |
| `paths.repo` | `${paths.src}/coda-rust` | The checkout inside the container |
| `carry.dir`, `carry.sourceRepo` | `/var/lib/coda-carry`, `~user/code/coda/coda-rust` | Where `coda-carry` writes, and what it reads |
| `broker.ghFallback` | `false` | Let the broker serve `gh auth token` when no `github-agent` is stored |
| `broker.keyringPackage` | `pkgs.libsecret` | Provides `secret-tool` for the broker |
| `ci.enable` | `false` | The Firecracker controller, its timer and the seed unit |
| `ci.guest`, `ci.workDiskSize`, `ci.marginMiB` | `{ }`, `200G`, `4096` | Guest configuration and sizing |
| `package` | this flake's | The binary installed on the host and in the container |

`paths.build` must support reflinks, and the module does not create filesystems: the host provides them and names the unit that mounts them in `requires`. `coda-sandbox doctor` checks the result.

## The agent container

`coda-agent` ([nix/container.nix](nix/container.nix) on [nix/nspawn.nix](nix/nspawn.nix)) has its own user namespace (the agent is an unmapped high host UID), a private network that cannot reach the host (except the ports `modelProxy` and `hostTcpPorts` open), the LAN or Tailscale, no sudo and no Docker. It sees only these host paths:

| Container path | Host path | Access |
| --- | --- | --- |
| `/agents/coda/src` | `/agents/coda/src` | read-write |
| `/agents/coda/build` | `/agents/coda/build` | read-write |
| `/agents/coda/ci/spool/requests` | `/agents/coda/ci/spool/requests` | drop-only (mode `1733`) |
| `/agents/coda/ci/spool/results` | `/agents/coda/ci/spool/results` | read-only |
| `/agents/coda/carry` | `/var/lib/coda-carry` | read-only |
| `/run/coda-sandbox/agent` | `/run/coda-sandbox/agent` | read-only; the token broker's socket |
| `/nix/store`, daemon socket | host store | read-only; the agent is an untrusted daemon user |

Paths equal the host's so the host Nix daemon's GC roots for each worktree's dev shell stay valid. Starting the container never gates boot: it is wanted only by `machines.target`, and if a unit in `requires` fails, only the container and the CI fail. `nix build ./crates/workspace-tools/sandbox#checks.x86_64-linux.agent-sandbox -L` runs the container on a late-imported ZFS pool in a throwaway VM and checks these boundaries.

Host git refuses trees the agent wrote (they belong to another UID). Do not mark them `safe.directory`: a hook or config the agent planted would run as you. Work on them from inside the container, or bring work out as a bundle.

### Agent sandbox credentials

No token sits in a file. The container's tokens live in your desktop keyring (the Secret Service, which KDE's wallet serves and PAM unlocks at login), and the token broker hands them out one request at a time. Create a fine-grained GitHub token limited to `tyler274/coda-rust` with **Contents: read and write** and **Pull requests: read and write**, and a read-only Doppler service token for `coda-rust`'s `dev` config. Store each once, as yourself:

```bash
coda-sandbox broker store github-agent   # paste at the hidden prompt, then Enter
coda-sandbox broker store doppler
coda-sandbox doctor                      # "token broker  github-agent: keyring; doppler: keyring"
```

`broker store` reads the token from standard input, never from the command line, and tells the running broker to read it on next use; `broker delete <item>` removes one. `coda-sandbox broker self-test` stores, reads back and deletes a throwaway item to prove the keyring works.

The broker ([nix/broker.nix](nix/broker.nix), [src/broker](src/broker)) is `coda-sandbox-broker.service`, a `systemd --user` service of `user` that keeps running while the user lingers. It listens on `/run/coda-sandbox/agent/broker.sock`, a directory bound into the container at the same path, and serves a secret only to a peer whose kernel-reported uid is the container's agent (`uidBase + 1000`); container root, other host users and other containers are refused, and you may ask it only for its report or a reload. It keeps each token in memory for an hour, keeps serving a cached one while the keyring is locked, rate-limits the container to 30 requests a minute, and writes one line per request to its journal (`journalctl --user -u coda-sandbox-broker`) with the caller, the outcome and an 8-digit SHA-256 fingerprint of the token, never the token.

Inside the container, git asks `coda-sandbox credential` for `https://github.com` (remotes written as `git@github.com:` are rewritten), and `gh` and `doppler` are wrappers that fetch their token for that one process, so no token is in the environment of a shell. With no answer, git fails rather than prompting. Anything in the container can ask, so the GitHub token must reach only `coda-rust`. `broker.ghFallback = true` lets the broker serve `gh auth token`, your general token, when no `github-agent` is stored; doctor warns while it is on or after it was used. Revoking a token on GitHub or Doppler is the kill switch.

With `ci.enable`, the broker also listens on `/run/coda-sandbox/ci/broker.sock`, in a directory only you and the `coda-ci` user can enter, and serves the `github-ci` token there to the CI controller's user alone. Each socket answers only for its own tokens: the container cannot ask for the CI token, even on its own socket, and the controller cannot ask for the container's. Each socket serves at most 16 connections at once, with separate room for you, and gives a caller two seconds to send its request, so a stalled caller on one socket cannot hold up the other, nor stop `broker store` from reaching the running broker. If the socket the controller would use exists but it may not reach it, its pass fails instead of quietly skipping GitHub.

The module's polkit rule lets `user` start, stop and restart `container@<name>.service` and the `coda-ci` units without sudo, so doctor's fixes for them are plain `systemctl`. A switch never restarts the container, so it cannot cut off an agent's work; a change to the container's own configuration takes effect at `systemctl restart container@<name>.service`.

### Working in the sandbox from an editor or agent CLI

Add an SSH entry for host `coda-agent` (`container.sshHost`): `HostName` is `localAddress`, `User agent`, a dedicated `IdentityFile` whose public key is in `authorizedKeys`, and `ForwardAgent no`. Then, on the host:

```bash
coda-sandbox open                      # the main checkout in Cursor
coda-sandbox open eng-7 --editor zed   # .worktrees/eng-7 in Zed
coda-sandbox shell                     # a login shell in the main checkout
coda-sandbox shell eng-7 -- claude     # Claude Code in that worktree's dev shell
```

`open` takes `--editor cursor`, `code`, `antigravity` or `zed`, and launches `cursor`, `code` or `antigravity-ide` with `--remote ssh-remote+coda-agent <dir>`, or `zeditor ssh://coda-agent<dir>` (on NixOS `zed` is the ZFS event daemon). `shell` runs a command through `direnv exec`, so it gets the repo's dev shell. Both refuse a worktree that does not exist, and pass `ForwardAgent=no` and `ClearAllForwardings=yes` to their own SSH. The editor window, and its own model sign-in, stay on the host; its remote server, terminals and agent commands run in the container.

Each checkout's `.envrc` is approved once in the container (`direnv allow`). Rust-analyzer and cargo are then the flake's, from the toolchain's own `rust-analyzer` component, started inside the checkout's dev shell:

- **Cursor and VS Code** need the direnv and rust-analyzer extensions on the container side. With `"remote.SSH.defaultExtensions": ["mkhl.direnv", "rust-lang.rust-analyzer"]` in the user settings, the remote server installs them on its first start; doctor warns when the key lacks either. The repo's `.vscode/settings.json` points the extension at the `rust-analyzer` on the dev shell's PATH.
- **Antigravity** has no such key. Install the two once in its remote server, from `coda-sandbox shell`: `~/.antigravity-ide-server/bin/*/bin/antigravity-ide-server --install-extension mkhl.direnv --install-extension rust-lang.rust-analyzer`, then reload the window. They stay installed until the server's directory is removed.
- **Zed** loads the dev shell through `load_direnv` in `.zed/settings.json`. It holds language servers back until the project is trusted, so the first open of each checkout asks once.

Nothing forwards a host credential: doctor checks that `ssh -G coda-agent` resolves `ForwardAgent no`, which is all Antigravity's remote and Zed have to go by, and that these settings are off where the editor has them: `remote.SSH.enableAgentForwarding`, `git.terminalAuthentication` and `github.gitAuthentication` for Cursor and VS Code, the two Git ones for Antigravity. An editor that has run here but has no settings file fails too, since every key is then at its default.

Claude Code and Codex run in the container, in `coda-sandbox shell` or an editor's terminal, and come from the dev shell. By default each signs in once there (`claude auth login`, `codex login`) and keeps its login in the agent's home.

With `container.modelProxy.enable`, both send their requests to a model proxy on the host instead, and no account sign-in enters the container. The proxy is [Coda's ungate](https://github.com/Coda-Health/ungate): it keeps the Claude and ChatGPT sign-ins as an account pool, passes Claude Code's `/v1/messages` and Codex's `/v1/responses` through to them, and records usage per client key. Run it on the host listening on `hostAddress` as well as loopback (its `HOST` takes a comma-separated list, and it binds `hostAddress` once the container's interface exists), on `container.modelProxy.port`, 47821 by default. Mint a client key for the container in its dashboard, then store it:

```bash
coda-sandbox broker store model-proxy
coda-sandbox doctor                   # "model proxy  http://10.233.1.1:47821 takes the key as client coda-agent"
```

The container's firewall opens that one port on `hostAddress` and keeps every other host port dropped; `container.hostTcpPorts` opens further ones, such as a local model server's. The option writes the container's system-wide config: Claude Code's `/etc/claude-code/managed-settings.json` sets `ANTHROPIC_BASE_URL` to the proxy and an `apiKeyHelper`, and `/etc/codex/config.toml` makes the proxy's Responses API Codex's provider with a key command. Both run `coda-sandbox token model-proxy` per session, so the key is in no file or environment. Managed settings outrank the agent's own, so every Claude Code session uses the proxy; the agent's `~/.codex/config.toml` can still pick another Codex provider. The proxy's admin routes answer only on loopback, so the container can use its key but cannot mint, list or revoke keys.

With the option on, `status` and doctor's token broker check list `model-proxy` and warn until it is stored, and doctor's model proxy check runs `coda-sandbox proxy-check` in the container over SSH: it asks the proxy's `GET /v1/client` with the stored key, which costs no tokens, and fails when the container cannot reach the proxy or the proxy refuses the key. A proxy with no client keys serves anyone who reaches it, so the check warns. Revoking the key in the dashboard cuts the container off.

The container is the sandbox for both CLIs. Codex's own `workspace-write` sandbox cannot start inside it, since its bubblewrap needs to mount `/proc`, so Codex runs commands there with `-s danger-full-access`, and the container still bounds what they reach.

`coda-sandbox worktree` owns worktrees and their build directories in the container, and a daily `coda-worktree-gc` timer there runs `coda-sandbox worktree gc`.

On Cyrene's host, `/etc/coda-sandbox/config.toml` names the container's checkout, which the host user cannot read. Worktrees of the home checkout use [config/cyrene-host.toml](config/cyrene-host.toml), which names that checkout and a build root under `~/.cache/coda-build`:

```bash
coda-sandbox --config crates/workspace-tools/sandbox/config/cyrene-host.toml worktree new <slug>
```

### Bringing host worktrees over

`sudo -u <user> coda-carry` records every host worktree of `carry.sourceRepo` as refs, publishing nothing: `refs/carry/<slug>/head` for its commit and `refs/carry/<slug>/dirty` for a snapshot of uncommitted and untracked files, built with a temporary index so the worktree is untouched. It runs git as `user`, then installs one bundle into `carry.dir` with `ignored.txt` beside it, listing ignored files outside `target/`. Inside the container, `git fetch /agents/coda/carry/carry.bundle 'refs/carry/*:refs/carry/*'` makes all of it available to `coda-sandbox worktree new <slug> --from carry/<slug>/dirty`.

## Coda CI in Firecracker

`services.coda-sandbox.ci` ([nix/ci.nix](nix/ci.nix)) runs `coda-rust` CI jobs one at a time, each in a disposable Firecracker microVM ([nix/ci-guest.nix](nix/ci-guest.nix)). Every 15 seconds the controller, `coda-sandbox ci-controller` ([src/ci_controller](src/ci_controller/mod.rs)), takes at most one job from two queues, GitHub first:

- **GitHub**: a queued job of `tyler274/coda-rust` whose `runs-on` is `cyrene-coda`. The controller registers a just-in-time runner with that label and a unique name. Inside the guest the runner runs in the pinned `catthehacker/ubuntu:rust-24.04` image, which carries rustup and cargo as GitHub's runner image does, so jobs see the same environment as under `act`. The registration is deleted after the job, whatever happened.
- **Local**: an `act` request in `/agents/coda/ci/spool/requests`. `coda-ci act [<repo-path-or-commit>] [job]` on the host, and `coda-sandbox ci run [--job <job>]` in `coda-agent`, each write a git bundle and a job name there and wait for the result in `spool/results/<id>/{log,status}`. The host copies the bundle without opening it, and the guest runs `act -j <job>` offline, with no secrets and no cache server.

Each job gets a directory under `/agents/coda/ci/jobs/` holding a reflink clone of `seed/work.img` (Docker's images and a scratch area, so a job costs only the blocks it writes) and a read-only inputs disk. The guest has no other host path, no host secret, and the `fc-coda-ci` tap, whose traffic nftables lets out to the internet but not to the host, the LAN, Tailscale or other private ranges. Its output leaves only over the serial console, which ends with `CODA_JOB_EXIT=<status>`. A job has three hours. Whatever happens, the controller removes the job directory, and on its next start it also removes any leftover directory and idle runner registration.

The guest has 8 vCPUs and 12 GiB. A job starts only when the guest plus `ci.marginMiB` fit in `MemAvailable` plus the ZFS ARC the kernel can evict above its floor; otherwise it waits for a later pass, and on GitHub the job stays queued.

To enable it:

1. Set `ci.enable = true`, then switch.
2. Create a fine-grained token for `tyler274/coda-rust` with **Administration: write** (to register runners) and **Actions: read** (to see the queue), and store it in your keyring, as yourself:

   ```bash
   coda-sandbox broker store github-ci   # paste at the hidden prompt, then Enter
   ```

   The controller asks the token broker for it on each pass and it never enters the guest. Without a token, or while the keyring is locked and the broker holds no cached copy, only local requests run and the controller's journal says why.
3. Build the seed: `sudo systemctl start coda-ci-seed`. It boots the guest once to pull the pinned image and the runner into a fresh work disk, then makes it `seed/work.img`. Run it again to pick up a new image.
4. Check a local run: `coda-ci act ~/code/coda/coda-rust ci`.
5. Send the repository's `ci` job here by setting the Actions variable `CI_JOB_RUNNER=cyrene-coda`. Deleting the variable sends it back to Blacksmith.

Labels route jobs but do not authorize them: any workflow in the repository that names `cyrene-coda` runs here, including from pull requests by people with write access. Forks' pull requests run only after GitHub's approval for outside contributors. To stop, delete `CI_JOB_RUNNER`, then `sudo systemctl stop coda-ci.timer coda-ci`. `nix build ./crates/workspace-tools/sandbox#checks.x86_64-linux.coda-firecracker -L` runs the module in a throwaway VM with nested KVM: an offline act request end to end, the seed unchanged after a job, the host and private ranges unreachable from the job, and the units running `coda-sandbox ci-controller`. The controller's own behavior tests run with the crate's `cargo test`. `nix build ./crates/workspace-tools/sandbox#nixosConfigurations.coda-ci-guest.config.microvm.declaredRunner` builds the guest on its own.

## Checks

`nix flake check ./crates/workspace-tools/sandbox -L` runs everything, including both VM tests; they need `/dev/kvm`, so hosted CI (the `sandbox` job in [.github/workflows/ci.yml](../../../.github/workflows/ci.yml)) builds every check but those two.
