# coda-sandbox

The Coda agent sandbox: a hardened NixOS container where coding agents (and you, over Remote-SSH) work on `coda-rust`, and disposable Firecracker microVMs that run its CI. The crate is a member of the root Cargo workspace; this directory is also its own flake, apart from the product flake, which builds only this crate from the root manifest and lockfile, so a host deploys it without the product flake's toolchain or other crates. It provides:

- `packages.coda-sandbox`: the `coda-sandbox` binary ([src](src)), with `doctor` and `status`.
- `nixosModules.default`: `services.coda-sandbox` ([nix/default.nix](nix/default.nix)), which declares the container, the CI and `coda-carry`, and writes `/etc/coda-sandbox/config.toml` for the binary.
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
  ci.tokenFile = "/var/lib/coda-ci/github-token";
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
| `ci.enable` | `false` | The Firecracker controller, its timer and the seed unit |
| `ci.tokenFile` | `null` | Root-only GitHub token for the GitHub queue |
| `ci.guest`, `ci.workDiskSize`, `ci.marginMiB` | `{ }`, `200G`, `4096` | Guest configuration and sizing |
| `package` | this flake's | The binary installed on the host and in the container |

`paths.build` must support reflinks, and the module does not create filesystems: the host provides them and names the unit that mounts them in `requires`. `coda-sandbox doctor` checks the result.

## The agent container

`coda-agent` ([nix/container.nix](nix/container.nix) on [nix/nspawn.nix](nix/nspawn.nix)) has its own user namespace (the agent is an unmapped high host UID), a private network that cannot reach the host, the LAN or Tailscale, no sudo and no Docker. It sees only these host paths:

| Container path | Host path | Access |
| --- | --- | --- |
| `/agents/coda/src` | `/agents/coda/src` | read-write |
| `/agents/coda/build` | `/agents/coda/build` | read-write |
| `/agents/coda/ci/spool/requests` | `/agents/coda/ci/spool/requests` | drop-only (mode `1733`) |
| `/agents/coda/ci/spool/results` | `/agents/coda/ci/spool/results` | read-only |
| `/agents/coda/carry` | `/var/lib/coda-carry` | read-only |
| `/run/agent-secrets` | `/var/lib/agent-secrets/coda-agent` | read-only |
| `/nix/store`, daemon socket | host store | read-only; the agent is an untrusted daemon user |

Paths equal the host's so the host Nix daemon's GC roots for each worktree's dev shell stay valid. Starting the container never gates boot: it is wanted only by `machines.target`, and if a unit in `requires` fails, only the container and the CI fail. `nix build ./crates/workspace-tools/sandbox#checks.x86_64-linux.agent-sandbox -L` runs the container on a late-imported ZFS pool in a throwaway VM and checks these boundaries.

Host git refuses trees the agent wrote (they belong to another UID). Do not mark them `safe.directory`: a hook or config the agent planted would run as you. Work on them from inside the container, or bring work out as a bundle.

### Agent sandbox credentials

The agent's tokens are files the agent can read and not change. Create a fine-grained GitHub token limited to `Coda-Health/coda-rust` with **Contents: read and write** and **Pull requests: read and write**, and a Doppler service token for the `dev` config (read-only). Then, with the group `uidBase + 100` (655460 by default):

```bash
d=/var/lib/agent-secrets/coda-agent
sudo install -m 0440 -o root -g 655460 /dev/stdin $d/github-token   # paste, then Ctrl-D
sudo install -m 0440 -o root -g 655460 /dev/stdin $d/doppler-token  # paste, then Ctrl-D
```

Anything in the container can read `github-token`, so a general `gh auth token` there can push to every repository you can; use the fine-grained token. Inside, git uses the GitHub token over HTTPS (remotes written as `git@github.com:` are rewritten), and `GH_TOKEN` and `DOPPLER_TOKEN` are set for login shells. Revoking either token on GitHub or Doppler is the kill switch. `coda-sandbox doctor` lists these files as ones the token broker replaces.

### Working in the sandbox from Cursor

Add an SSH entry for host `coda-agent`: `HostName` is `localAddress`, `User agent`, a dedicated `IdentityFile` whose public key is in `authorizedKeys`, and `ForwardAgent no`. Connect with Remote-SSH and open `/agents/coda/src/coda-rust`. Keep these Cursor settings off so no host credential is forwarded: `remote.SSH.enableAgentForwarding`, `git.terminalAuthentication`, `github.gitAuthentication`; `coda-sandbox doctor` flags them. `coda-sandbox worktree` owns worktrees and their build directories there, and a daily `coda-worktree-gc` timer in the container runs `coda-sandbox worktree gc`.

### Bringing host worktrees over

`sudo -u <user> coda-carry` records every host worktree of `carry.sourceRepo` as refs, publishing nothing: `refs/carry/<slug>/head` for its commit and `refs/carry/<slug>/dirty` for a snapshot of uncommitted and untracked files, built with a temporary index so the worktree is untouched. It runs git as `user`, then installs one bundle into `carry.dir` with `ignored.txt` beside it, listing ignored files outside `target/`. Inside the container, `git fetch /agents/coda/carry/carry.bundle 'refs/carry/*:refs/carry/*'` makes all of it available to `coda-sandbox worktree new <slug> --from carry/<slug>/dirty`.

## Coda CI in Firecracker

`services.coda-sandbox.ci` ([nix/ci.nix](nix/ci.nix)) runs `coda-rust` CI jobs one at a time, each in a disposable Firecracker microVM ([nix/ci-guest.nix](nix/ci-guest.nix)). Every 15 seconds the controller ([nix/coda_ci.py](nix/coda_ci.py)) takes at most one job from two queues, GitHub first:

- **GitHub**: a queued job of `Coda-Health/coda-rust` whose `runs-on` is `cyrene-coda`. The controller registers a just-in-time runner with that label and a unique name. Inside the guest the runner runs in the pinned `catthehacker/ubuntu:rust-24.04` image, which carries rustup and cargo as GitHub's runner image does, so jobs see the same environment as under `act`. The registration is deleted after the job, whatever happened.
- **Local**: an `act` request in `/agents/coda/ci/spool/requests`. `coda-ci act [<repo-path-or-commit>] [job]` on the host, and `coda-sandbox ci run [--job <job>]` in `coda-agent`, each write a git bundle and a job name there and wait for the result in `spool/results/<id>/{log,status}`. The host copies the bundle without opening it, and the guest runs `act -j <job>` offline, with no secrets and no cache server.

Each job gets a directory under `/agents/coda/ci/jobs/` holding a reflink clone of `seed/work.img` (Docker's images and a scratch area, so a job costs only the blocks it writes) and a read-only inputs disk. The guest has no other host path, no host secret, and the `fc-coda-ci` tap, whose traffic nftables lets out to the internet but not to the host, the LAN, Tailscale or other private ranges. Its output leaves only over the serial console, which ends with `CODA_JOB_EXIT=<status>`. A job has three hours. Whatever happens, the controller removes the job directory, and on its next start it also removes any leftover directory and idle runner registration.

The guest has 8 vCPUs and 12 GiB. A job starts only when the guest plus `ci.marginMiB` fit in `MemAvailable` plus the ZFS ARC the kernel can evict above its floor; otherwise it waits for a later pass, and on GitHub the job stays queued.

To enable it:

1. Create a fine-grained token for `Coda-Health/coda-rust` with **Administration: write** (to register runners) and **Actions: read** (to see the queue), and place it in a root-only file:

   ```bash
   sudo install -d -m 0700 /var/lib/coda-ci
   sudo install -m 0400 /dev/stdin /var/lib/coda-ci/github-token   # paste, then Ctrl-D
   ```

   Without a token only local requests run.
2. Set `ci.enable = true` and `ci.tokenFile = "/var/lib/coda-ci/github-token"`, then switch. The token reaches the controller as a systemd credential and never enters the guest.
3. Build the seed: `sudo systemctl start coda-ci-seed`. It boots the guest once to pull the pinned image and the runner into a fresh work disk, then makes it `seed/work.img`. Run it again to pick up a new image.
4. Check a local run: `coda-ci act ~/code/coda/coda-rust ci`.
5. Send the repository's `ci` job here by setting the Actions variable `CI_JOB_RUNNER=cyrene-coda`. Deleting the variable sends it back to Blacksmith.

Labels route jobs but do not authorize them: any workflow in the repository that names `cyrene-coda` runs here, including from pull requests by people with write access. Forks' pull requests run only after GitHub's approval for outside contributors. To stop, delete `CI_JOB_RUNNER`, then `sudo systemctl stop coda-ci.timer coda-ci`. `nix build ./crates/workspace-tools/sandbox#checks.x86_64-linux.coda-firecracker -L` runs the module in a throwaway VM with nested KVM: an offline act request end to end, the seed unchanged after a job, the host and private ranges unreachable from the job, and the controller's unit tests. `nix build ./crates/workspace-tools/sandbox#nixosConfigurations.coda-ci-guest.config.microvm.declaredRunner` builds the guest on its own.

## Checks

`nix flake check ./crates/workspace-tools/sandbox -L` runs everything, including both VM tests; they need `/dev/kvm`, so hosted CI (the `sandbox` job in [.github/workflows/ci.yml](../../../.github/workflows/ci.yml)) builds every check but those two.
