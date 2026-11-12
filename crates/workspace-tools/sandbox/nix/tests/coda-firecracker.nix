# Regression test for Coda CI in Firecracker:
#   nix build ./crates/workspace-tools/sandbox#checks.x86_64-linux.coda-firecracker -L
# Runs the real ci.nix in a throwaway VM with nested KVM and no internet: a
# stub stands in for the GitHub API, a stub `secret-tool` for the keyring
# the token broker reads, an XFS disk for the agents pool (both
# clone with reflinks), and an image built here for the job image.
{ sandboxModule }:
{ pkgs, lib, ... }:
let
  keyring = "/var/lib/fake-keyring";

  # Enough of a runner image for act's run steps, built offline.
  jobImage = pkgs.dockerTools.buildImage {
    name = "coda-test";
    tag = "latest";
    copyToRoot = pkgs.buildEnv {
      name = "coda-test-root";
      paths = with pkgs; [ bashInteractive coreutils curl gnugrep ];
      pathsToLink = [ "/bin" ];
    };
    extraCommands = ''
      mkdir -m 1777 tmp
      mkdir -p usr/bin
      ln -s /bin/env usr/bin/env    '';
    config.Env = [ "PATH=/bin" ];
  };

  workflow = pkgs.writeText "ci.yml" ''
    on: push
    jobs:
      ci:
        runs-on: blacksmith-4vcpu-ubuntu-2404
        steps:
          - run: echo CODA_TEST_JOB_RAN
          # Writes to the job's work disk, which must not reach the seed.
          - run: head -c 32M /dev/urandom > scratch.bin && sync
          - run: |
              curl --version >/dev/null
              for target in http://10.234.0.1:8080/ http://10.0.2.15:8080/ http://100.100.100.100/; do
                rc=0
                curl -s -m 3 -o /dev/null "$target" || rc=$?
                echo "CODA_PROBE $target $rc"
                # 28 is curl's timeout: the packets were dropped, not refused.
                [ "$rc" -eq 28 ]
              done
      fails:
        runs-on: ubuntu-latest
        steps:
          - run: exit 3
  '';

  githubStub = pkgs.writeText "github-stub.py" ''
    import http.server
    import json

    class Handler(http.server.BaseHTTPRequestHandler):
        def do_GET(self):
            with open("/var/log/github-stub.log", "a") as log:
                log.write(f"{self.path} {self.headers.get('Authorization')}\n")
            body = {"runners": []} if "/actions/runners" in self.path else {"workflow_runs": []}
            data = json.dumps(body).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

    http.server.HTTPServer(("127.0.0.1", 8099), Handler).serve_forever()
  '';
in
{
  name = "coda-firecracker";

  nodes.machine =
    { config, pkgs, ... }:
    {
      imports = [ sandboxModule ];

      virtualisation = {
        memorySize = 6144;
        cores = 4;
        qemu.options = [ "-cpu host" ];
        emptyDiskImages = [
          {
            size = 16384;
            driveConfig.deviceExtraOpts.serial = "ci";
          }
        ];
      };
      networking.nftables.enable = true;

      # The CI alone: no agent container in this VM.
      services.coda-sandbox = {
        enable = true;
        user = "alice";
        requires = [ "agents-pool.service" ];
        container.enable = false;
      };
      services.coda-sandbox.ci = {
        enable = true;
        apiUrl = "http://127.0.0.1:8099";
        # The stub listens on loopback, which Cyrene's controller may not use.
        controllerAddressDeny = [ "10.0.0.0/8" "100.64.0.0/10" ];
        marginMiB = 256;
        workDiskSize = "6G";
        guest = {
          microvm.mem = 2048;
          microvm.vcpu = 2;
          codaCiGuest = {
            image = "coda-test:latest";
            seedArchives = [ jobImage ];
            seedRunner = false;
          };
        };
      };
      # No desktop session runs a Secret Service here; the stub keeps items
      # as files the broker's user owns.
      services.coda-sandbox.broker.keyringPackage = pkgs.callPackage ./fake-secret-tool.nix { dir = keyring; };
      systemd.tmpfiles.rules = [ "d ${keyring} 0700 alice users -" ];

      # Stands in for the agents pool: a reflink-capable disk at the CI path.
      systemd.services.agents-pool = {
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
        };
        path = with pkgs; [ xfsprogs util-linux ];
        script = ''
          disk=/dev/disk/by-id/virtio-ci
          blkid "$disk" || mkfs.xfs -q "$disk"
          mkdir -p /agents/coda/ci
          mountpoint -q /agents/coda/ci || mount "$disk" /agents/coda/ci
        '';
      };

      systemd.services.github-stub = {
        wantedBy = [ "multi-user.target" ];
        before = [ "coda-ci.service" ];
        serviceConfig.ExecStart = "${pkgs.python3}/bin/python3 ${githubStub}";
      };

      # A host service the guest must not reach.
      services.nginx = {
        enable = true;
        virtualHosts.default.listen = [ { addr = "0.0.0.0"; port = 8080; } ];
      };
      networking.firewall.allowedTCPPorts = [ 8080 ];

      # Lingering, as on Cyrene, so the broker runs without a login.
      users.users.alice = {
        isNormalUser = true;
        linger = true;
      };
      environment.systemPackages = [ pkgs.git ];
    };

  testScript = ''
    import re

    def as_alice(cmd):
        return f"su - alice -c '{cmd}'"

    def broker(cmd):
        return machine.succeed(as_alice("export XDG_RUNTIME_DIR=/run/user/$(id -u); " + cmd))

    local_only = "journalctl -u coda-ci --no-pager | grep -c 'no GitHub token, so only local requests run' || true"

    machine.start()
    machine.wait_for_unit("multi-user.target")

    with subtest("the units run coda-sandbox ci-controller"):
        for unit, args in (("coda-ci", "ci-controller"), ("coda-ci-seed", "ci-controller seed")):
            exec_start = machine.succeed(f"grep '^ExecStart=' /etc/systemd/system/{unit}.service").strip()
            assert re.fullmatch(rf"ExecStart=/nix/store/[^/ ]+-coda-sandbox-[^/ ]+/bin/coda-sandbox {args}", exec_start), exec_start

    with subtest("nested KVM and the CI spool"):
        machine.succeed("test -c /dev/kvm")
        machine.wait_for_unit("agents-pool.service")
        machine.wait_for_unit("coda-ci-spool.service")
        machine.succeed("test \"$(stat -c %a /agents/coda/ci/spool/requests)\" = 1733")

    with subtest("the broker serves the CI token on a socket only coda-ci reaches"):
        machine.wait_for_unit("coda-sandbox-broker.service", "alice")
        machine.wait_for_file("/run/coda-sandbox/ci/broker.sock")
        mode = machine.succeed("stat -c '%a %U %G' /run/coda-sandbox/ci").strip()
        assert mode == "750 alice coda-ci", mode
        machine.fail("su -s /bin/sh nobody -c 'test -e /run/coda-sandbox/ci/broker.sock'")
        machine.succeed("grep -q '^Environment=.*CODA_CI_BROKER_SOCKET=/run/coda-sandbox/ci/broker.sock' /etc/systemd/system/coda-ci.service")
        machine.fail("grep -q LoadCredential /etc/systemd/system/coda-ci.service")
        out = broker("printf %s test-token | coda-sandbox broker store github-ci")
        assert "running broker" in out, out
        machine.succeed("test \"$(cat ${keyring}/github-ci)\" = test-token")

    with subtest("seed: a fresh work disk with the job image"):
        machine.succeed("systemctl start coda-ci-seed.service")
        machine.succeed("test -f /agents/coda/ci/seed/work.img")
        seed = machine.succeed("sha256sum < /agents/coda/ci/seed/work.img")
        machine.succeed("journalctl -u coda-ci-seed --no-pager | grep -q 'New seed'")

    with subtest("tap and egress rules exist"):
        machine.succeed("ip -br addr show fc-coda-ci | grep -q 10.234.0.1/30")
        machine.succeed("nft list table inet coda-ci")
        machine.succeed("nft list table ip nixos-nat | grep -q fc-coda-ci")

    with subtest("an act request runs offline, end to end"):
        machine.succeed(as_alice(
            "git init -q repo && mkdir -p repo/.github/workflows"
            " && cp ${workflow} repo/.github/workflows/ci.yml"
            " && git -C repo add -A"
            " && git -C repo -c user.name=t -c user.email=t@t commit -qm ci"
        ))
        out = machine.succeed(as_alice("cd repo && timeout 900 coda-ci act . ci") + " 2>&1")
        print(out)
        assert "CODA_TEST_JOB_RAN" in out, "job output missing"
        assert out.count("CODA_PROBE") >= 3, "egress probes missing"
        assert "CODA_JOB_EXIT=0" in out, "exit marker missing"
        queued = re.search(r"coda-ci: queued ([0-9a-f]{12})", out)
        assert queued is not None, "request id missing"
        request = queued.group(1)
        machine.succeed(f"grep -qx 0 /agents/coda/ci/spool/results/{request}/status")
        machine.succeed(f"journalctl -u coda-ci --no-pager | grep -q 'Local request {request} exited 0'")

    with subtest("the GitHub queue was checked first"):
        requests = machine.succeed("cat /var/log/github-stub.log")
        assert "actions/runners?per_page=100" in requests, requests
        assert "actions/runs?status=queued" in requests, requests
        assert "Bearer test-token" in requests, requests
        assert "test-token" not in machine.succeed("journalctl --no-pager"), "the token reached the journal"

    with subtest("nothing of the job is left, and the seed is unchanged"):
        machine.succeed("test -z \"$(ls -A /agents/coda/ci/jobs)\"")
        machine.fail("pgrep -x firecracker")
        assert machine.succeed("sha256sum < /agents/coda/ci/seed/work.img") == seed

    with subtest("a failing job reports failure"):
        machine.succeed(as_alice("cd repo && rc=0; timeout 900 coda-ci act . fails || rc=$?; test $rc -eq 1"))

    with subtest("without a stored token only local requests run"):
        broker("coda-sandbox broker delete github-ci")
        machine.succeed("systemctl stop coda-ci.timer coda-ci.service", ": > /var/log/github-stub.log")
        warned = int(machine.succeed(local_only))
        machine.succeed("systemctl start coda-ci.timer")

    with subtest("an invalid request is rejected without a guest"):
        machine.succeed(
            "echo junk > /agents/coda/ci/spool/requests/abcdefabcdef.bundle",
            "echo 'bad name' > /agents/coda/ci/spool/requests/abcdefabcdef.job",
        )
        machine.wait_until_succeeds("test -e /agents/coda/ci/spool/results/abcdefabcdef/status", timeout=120)
        machine.succeed("grep -qx 125 /agents/coda/ci/spool/results/abcdefabcdef/status")
        machine.succeed("grep -q 'job name' /agents/coda/ci/spool/results/abcdefabcdef/log")
        assert int(machine.succeed(local_only)) > warned, "no local-only warning"
        assert machine.succeed("cat /var/log/github-stub.log") == "", "GitHub was asked without a token"
  '';
}
