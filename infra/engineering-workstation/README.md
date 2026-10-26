# Personal engineering workstation

This recipe captures Dev's working Ubuntu desktop on GCP: Codex scheduled tasks,
Claude Desktop, Chrome, Docker, and this repository's Nix development environment.
It is separate from the product infrastructure under `infra/gcp`. Nothing here
initializes or applies a product infrastructure root.

The existing machine was installed and exercised interactively. These extracted
scripts and templates have syntax/configuration checks; a fresh VM has **not** yet
been rebuilt end to end from this directory. The installation uses current vendor
packages, so it is a repeatable procedure, not a bit-for-bit image. No new cloud
resources are created merely by adding this recipe.

## Ownership

| Concern | Owner |
| --- | --- |
| VM, disk, network, IAP, OS Login, identity and warmup | GCP engineering project; operator setup below |
| Ubuntu packages and desktop services | This directory's host scripts and templates |
| Rust, Node, Python and repository tools | Root `flake.nix` and `flake.lock`; use `nix develop` |
| Projects, plugins, MCP registration and scheduled tasks | Native Codex app, through its supported UI/tools |
| ChatGPT/Claude/GitHub sign-in and private state | The individual user and the persistent home directory |

Do not copy a signed-in home directory, browser profile, keyring, token, service
account key, `.codex` database, or application cookies into an image or this repo.
Plugin names and prompts are reproducible inputs; account grants and saved runs
are private state. A stopped VM retains its disk; this is not a backup strategy.

## Current Dev profile

Verified October 25, 2026 (Eastern):

| Field | Value |
| --- | --- |
| Project / zone | `coda-engineering` / `us-east1-b` |
| VM / owner label | `dev-patale-workstation` / `owner=dev_patale` |
| Stable instance ID | `4760457100108491227` |
| OS / machine / disk | Ubuntu 24.04 amd64 / `e2-standard-8` / 200 GB `pd-balanced` |
| Existing boot disk | `codex-workstation`; retained across the VM rename, also labeled `owner=dev_patale` |
| OS Login user / checkout | `dev_codahealth_ai` / `/home/dev_codahealth_ai/code/coda-rust` |
| Network / subnet | `engineering` / `engineering-us-east1` |
| SSH aliases on Dev's Mac | `dev-patale-workstation` and compatibility alias `coda-engineering` |
| Attached audit identity | `coda-rust-security-auditor@coda-engineering.iam.gserviceaccount.com` |
| Timezone | `America/New_York` |

The VM was renamed in place. Its ID, boot disk, service account, instance IAM and
warmup policy were retained. The native security task's VM-name reference was
updated through the app. Do not rename the disk just to match the VM: its current
name is an identity reference, not a second workstation.

The access review found `dev@codahealth.ai` as the only human principal with
current project access. Organization and IAP policies were also inspected; no
additional human login grant was found. Google infrastructure service agents and
the metadata-only audit identity remain. This is a dated IAM observation, not a
promise that administrators can never change access. The owner label is for
identification; IAM enforces access.

SSH uses OS Login through IAP. Project SSH keys are blocked. The effective ingress
rule allows TCP 22 only from `35.235.240.0/20` to the engineering workstation tag.
VNC and noVNC bind to loopback; neither port is exposed by a cloud firewall.

## 1. Prepare an approved VM

For another owner or replacement machine, choose a unique `<owner>-workstation`
name and an `owner=<owner_slug>` label. Set the project, region, zone, account,
network and service account explicitly. Provisioning another VM or granting a
new principal access is a separate operator action; this document does not grant
that authority.

Use Ubuntu 24.04 amd64, Shielded VM secure boot/vTPM/integrity monitoring, OS Login,
blocked project SSH keys, and an engineering-only network with IAP SSH ingress.
The exercised image was `ubuntu-2404-noble-amd64-v20261024` from `ubuntu-os-cloud`.
The current machine uses an ephemeral Standard-tier external IP for outbound
internet access; inbound access remains firewall-restricted. Do not remove it
without arranging the outbound connectivity needed by package managers and apps.

Example VM creation, **after** an operator prepares the network, scoped access
and dedicated service account and approves the new resource:

```sh
gcloud compute instances create "$workstation_name" \
  --project="$engineering_project" --zone="$workstation_zone" \
  --machine-type=e2-standard-8 \
  --image=ubuntu-2404-noble-amd64-v20261024 --image-project=ubuntu-os-cloud \
  --boot-disk-size=200GB --boot-disk-type=pd-balanced \
  --network="$workstation_network" --subnet="$workstation_subnet" \
  --network-tier=STANDARD --tags=engineering-workstation \
  --metadata=enable-oslogin=TRUE,block-project-ssh-keys=TRUE,disable-legacy-endpoints=TRUE \
  --shielded-secure-boot --shielded-vtpm --shielded-integrity-monitoring \
  --service-account="$audit_account" --scopes=cloud-platform \
  --labels="environment=engineering,owner=$owner_slug,product=coda-rust,purpose=engineering-workstation"
```

Do not run this against an existing machine. Do not grant project Owner merely to
copy this setup. Human access should be scoped to the intended owner using IAP,
OS Login and the required service-account access. Verify inherited organization,
folder, project, instance and IAP policies, effective firewall rules, and actual
login. An OAuth scope is not a substitute for a constrained IAM role.

## 2. Install host packages and the private desktop

Copy this directory to the approved VM. Log in once using OS Login so the user's
home exists. Run [install-host.sh](install-host.sh) as root. It installs the
Ubuntu desktop dependencies, official ChatGPT/Chrome packages, Anthropic's signed
APT repository, Nix and Google's Cloud CLI snap. It checks Anthropic's signing
key fingerprint before enabling that repository. It does not sign in to apps.

```sh
sudo bash infra/engineering-workstation/install-host.sh
mkdir -p ~/.vnc
chmod 700 ~/.vnc
tigervncpasswd ~/.vnc/passwd
sudo bash infra/engineering-workstation/install-desktop.sh "$(id -un)"
```

The owner enters the VNC password directly; do not print or put it in a command,
recipe or chat. [install-desktop.sh](install-desktop.sh) substitutes the actual
OS Login user/home/UID into the service templates, enables user lingering,
sets Eastern time, adds the owner to Docker, and starts the private desktop.
Docker membership is root-equivalent host access and belongs only to the approved
owner. The script refuses to overwrite a running desktop's configuration.

The two system services are `coda-desktop` (TigerVNC/XFCE on localhost:5901) and
`coda-desktop-web` (noVNC on localhost:6080). ChatGPT starts through the user's
XFCE autostart entry. Allow the account session and keyring to initialize before
signing in. Do not bypass an app's credential storage with plaintext files.

Claude Desktop uses Anthropic's official repository. Dev chose Desktop and Claude
Code on the existing E2 VM. Cowork's nested-virtualization requirements are not
met by this machine; do not change its machine type to enable Cowork as part of
this recipe. Claude was installed and launch-verified; user sign-in remains.

The host installer is provisioning, not the weekly updater. Review package
changes before rerunning it on an established machine; it may upgrade the host
packages it names. Existing native apps are left to their package owner and the
maintenance policy below. No duplicate host Rust/Node toolchain is installed.

The existing VM still has its original one-time startup bootstrap and
`/var/lib/coda-engineering/bootstrap-complete` marker. That bootstrap does not
reconcile later changes and has not been replaced by these extracted scripts.
Use this versioned recipe for a future approved build; applying recipe changes
to an existing desktop remains an explicit maintenance operation.

## 3. Connect from the owner's computer

Use the owner's existing authenticated `gcloud` installation on their computer:

```sh
gcloud compute ssh "$workstation_name" \
  --project="$engineering_project" --zone="$workstation_zone" --tunnel-through-iap \
  -- -N -L 127.0.0.1:15901:127.0.0.1:5901 -L 127.0.0.1:16080:127.0.0.1:6080
```

Open `vnc://127.0.0.1:15901` in Screen Sharing, or
`http://127.0.0.1:16080/vnc.html` locally. Retain strict SSH host-key checking.
For a permanent SSH alias, use the numeric instance ID for `HostKeyAlias` and
the VM name in the `gcloud compute start-iap-tunnel` ProxyCommand. Renaming a VM
requires updating the ProxyCommand; its numeric identity remains stable.

Dev's Mac has a LaunchAgent at
`~/Library/LaunchAgents/ai.coda.engineering-desktop-tunnel.plist` using the
`coda-engineering` alias with these two loopback forwards. It reconnects after VM
startup. Its saved Screen Sharing connection works with the renamed VM. Neither
the tunnel nor the Mac executes Linux scheduled tasks.

## 4. Register the workspace and capabilities

Sign in to ChatGPT/Codex and GitHub as the intended owner using their normal
flows. Do not transfer another user's auth files. Clone
`https://github.com/Coda-Health/coda-rust.git` into `~/code/coda-rust`, then open
that folder as a native **local** project in Codex. Initialize submodules only
when needed by the actual workflow; `coda-agents/eval` stays sealed.

```sh
cd ~/code/coda-rust
nix develop --command bash -c 'rustc --version; cargo --version; node --version; uv --version; act --version'
docker info
```

Use the desktop's bundled Codex at `/usr/lib/chatgpt/resources/codex`. The current
host has `/usr/local/bin/codex` pointing there. Preserve that owner instead of
installing a parallel npm Codex. Product CLIs come from `nix develop`.

Install plugins through Codex's supported marketplace UI/tools. The capabilities
used here are Codex app tools, Browser, Unified Computer Use, Visualize, document/
PDF/spreadsheet/presentation tools, Codex Security and GitHub. Let the desktop
manage its bundled plugins, node-repl environment and native browser bridge; do
not copy versioned cache paths or another computer's generated configuration.
Connected cloud apps and local runtime capabilities are distinct: verify the
actual tool operation on this host after registration.

A standalone Chrome DevTools MCP is also installed in the user's writable
`~/.local/share/coda-engineering/browser-tools`, with `chrome-devtools-mcp` pinned
to `1.12.0` in its package/lock files. Install that exact package using the repo's
Nix-provided npm, into that user directory (not an immutable Nix global prefix).
Register it through supported MCP configuration with this observed command:

```text
/usr/bin/env -u LD_LIBRARY_PATH /usr/lib/chatgpt/resources/cua_node/bin/node
  <OWNER_HOME>/.local/share/coda-engineering/browser-tools/node_modules/chrome-devtools-mcp/build/src/bin/chrome-devtools-mcp.js
  --headless --isolated --executable-path=/usr/bin/google-chrome
  --no-usage-statistics --no-performance-crux
```

Set its `CHROME_DEVTOOLS_MCP_NO_UPDATE_CHECKS=1`. Verify initialization/tool listing
and a synthetic browser navigation/screenshot. An MCP config entry alone does not
prove connection. Unsetting `LD_LIBRARY_PATH` avoids passing the Nix shell's
library overrides into the vendor Chrome binary.

## 5. Recreate native tasks and keyless audit access

| Native task | Local time | Model / reasoning | Prompt |
| --- | --- | --- | --- |
| Coda Rust code and live infrastructure security | Sunday–Friday 11:00 | `gpt-6-astra` / `xhigh` | [security-review.md](prompts/security-review.md) |
| Update Linux dev tools | Sunday 10:40 | `gpt-6.1-sol` / `xhigh` | [toolchain-sweep.md](prompts/toolchain-sweep.md) |

Replace `<OWNER_HOME>` and `<VM_NAME>` before creating the tasks in the native
app or its supported automation tool. Use the newly registered local project's
ID, a fresh chat per run, all-run notifications and no automatic archival of
successful results. Verify model, timezone, cadence, project and saved prompt.
Do not copy automation databases/TOML or invent a cron/CLI substitute. Keep the
Mac security task paused after native acceptance. Dev's Mac cleanup proposal
audit and Mac tool updater remain on the Mac.

For this Coda audit, [audit/gcp-metadata-role.json](audit/gcp-metadata-role.json)
records the approved custom role. The existing keyless audit service account has
that role in `coda-engineering`, `coda-health-staging` and `coda-health-prod`.
It has no secret-payload, bucket-object, log-entry, database-connect,
impersonation or mutation permissions. Inspect configuration with field selection:
some otherwise allowed APIs can contain inline values. Additional projects need
their own access decision; inaccessible resources remain explicit coverage gaps.

[audit/doppler-metadata.py](audit/doppler-metadata.py) records the existing Coda
OIDC integration. Its identity ID, audience and service-account email are nonsecret
configuration. For another audit identity, configure and review
the corresponding Doppler trust and update those constants before using it.
The exchange uses a short-lived Google identity token and keeps the returned
Doppler token in memory. Its role permits selected workplace metadata and the
None project role for `coda-rust`; secret values and some membership/rotation
details are deliberately unavailable. Never provision personal GCP or Doppler
credentials onto the audit VM to fill a coverage gap.

Install the selected helpers under the owner's `~/coda-audit/`. Keep prompts and
helpers consistent with the chosen paths. Do not copy historical scan reports to
make a new machine look validated: its first run must establish its own evidence.

## 6. Warmup and completion-driven shutdown

The current `codex-audit-warmup` Compute resource policy starts the VM at 10:30
Eastern Sunday–Friday. It has **no fixed stop schedule**. Its Compute service
agent has only `compute.instances.start` and `compute.instances.stop` through
custom role `codexVmScheduler` bound to this VM. The VM-attached audit identity
does not receive those cloud permissions. Reuse the existing policy for this VM;
for another machine, review the time and attach the policy explicitly.

The first scheduled warmup had not yet occurred when this recipe was captured.
Configuration readback is established; timed execution still needs observation.

[audit/audit-shutdown.py](audit/audit-shutdown.py) is the exercised completion
helper, with its home directory derived from the executing user. Set `AUTOMATION`
to the actual native security task ID. The current ID is
`coda-rust-code-and-live-infrastructure-security`. Merge this into the **user-level**
Codex config, preserving other fields and any existing notification integration:

```toml
notify = ["/usr/bin/python3", "<OWNER_HOME>/coda-audit/audit-shutdown.py"]
```

Start with `/etc/coda-audit/shutdown-enabled` absent. A real native completion
callback schedules one check after the normal inbox save. It checks the exact
native run/turn, saved result and canonical sealed source artifacts; confirms no
other unfinished native task; and vetoes shutdown for a desktop viewer, Docker
container, Claude process, build/test, Codex CLI execution or blocking shutdown
inhibitor. Failure leaves the VM running. There is no periodic shutdown loop and
no deadline interpreted as success. These are point-in-time checks, not an atomic
guarantee against arbitrary work starting immediately afterward.

This helper depends on the current native automation database schema and bundled
app-server read API. Revalidate it after app/schema changes. The native app owns
the runs; the helper only reads them. It never starts a model turn or writes a
second run ledger. It requires noninteractive permission to request OS poweroff;
the current owner already has OS Admin Login. Do not grant the audit service
account broad cloud permissions to power off a guest OS.

After one **real callback** passes in dry-run mode, enable shutdown with:

```sh
sudo install -d -m 755 /etc/coda-audit
sudo install -m 644 /dev/null /etc/coda-audit/shutdown-enabled
```

Inspect logs with `sudo journalctl -t coda-audit-shutdown`; `journalctl --user`
does not show these logger records. `--check THREAD_ID TURN_ID` always performs a
dry run. `--event-check` can power off when the marker exists. Disable automatic
shutdown for maintenance by removing only the marker; do not interrupt active
work. A busy-work veto has no automatic retry, so the VM can remain on until an
operator finishes the work or a later genuine audit completion.

## Acceptance and recovery

On a new machine, verify these in order before calling the setup reusable:

1. IAM/firewall/OS Login and IAP access; loopback-only desktop listeners.
2. Desktop and native Codex sign-in after reboot with the viewer disconnected.
3. Repository identity, Nix startup, Docker and representative local CI; no
   production secrets or patient data in the checks.
4. Native plugins/MCPs through an actual tool call, including a browser screenshot.
5. Read-only cloud metadata and Doppler checks; expected refusals remain refusals.
6. One native audit with saved inbox/final answer and verified sealed artifacts;
   one tool sweep with updates/holds reported honestly.
7. Real completion-callback dry run, active-work veto, enabled OS shutdown, and
   GCP `TERMINATED` readback. Verify the next scheduled warmup separately.

For Dev's existing machine these paths were exercised before extracting this
recipe, including a real callback dry run and an enabled same-run shutdown check.
The rename was additionally checked with live SSH, the desktop UI and native task
readback. The complete recipe still needs one authorized disposable rebuild.

The durable disk holds checkout changes, native tasks, reports, app state and
sign-ins. Reinstallation instructions do not recover unbacked-up private state.
Before replacement, preserve needed user data through the approved backup
process; never delete the current disk as an installation step.

## References

- [GCP VM rename semantics](https://docs.cloud.google.com/compute/docs/instances/rename-instance)
- [IAP TCP forwarding](https://docs.cloud.google.com/iap/docs/using-tcp-forwarding)
- [Claude Desktop on Linux](https://code.claude.com/docs/en/desktop-linux)
- [Codex notifications](https://learn.chatgpt.com/docs/config-file/config-advanced#notifications)
- [Codex app-server API](https://learn.chatgpt.com/docs/app-server)
