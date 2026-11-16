# The coda-sandbox half of the agent-sandbox check, appended to the test
# script in agent-sandbox.nix. Paths are the module defaults, which
# config/cyrene.toml repeats: each names the same directory on the host and
# inside the container.

in_container = "nixos-container run coda-agent -- "

with subtest("binds: carry and CI results read-only, requests drop-only"):
    machine.succeed("echo bundle > /var/lib/coda-carry/probe")
    agent("cat /agents/coda/carry/probe")
    agent_fails("touch /agents/coda/carry/x")
    agent_fails("touch /agents/coda/ci/spool/results/x")
    # Drop-only: the agent cannot list or remove other requests.
    machine.succeed("touch /agents/coda/ci/spool/requests/other")
    agent_fails("ls /agents/coda/ci/spool/requests")
    agent_fails("rm -f /agents/coda/ci/spool/requests/other")
    machine.succeed("test -e /agents/coda/ci/spool/requests/other")
    agent_fails("ls /agents/coda/ci/seed")

with subtest("spool: what the agent drops is the file the host sees"):
    agent("echo from-agent > /agents/coda/ci/spool/requests/probe.job")
    seen = machine.succeed("cat /agents/coda/ci/spool/requests/probe.job").strip()
    assert seen == "from-agent", seen
    machine.succeed("echo from-host > /agents/coda/ci/spool/results/probe.json")
    got = agent("cat /agents/coda/ci/spool/results/probe.json").strip()
    assert got == "from-host", got

with subtest("coda-sandbox and the module's config on the host and inside"):
    host_config = machine.succeed("cat /etc/coda-sandbox/config.toml")
    inside_config = machine.succeed(in_container + "cat /etc/coda-sandbox/config.toml")
    assert host_config == inside_config, (host_config, inside_config)
    for out in (machine.succeed("coda-sandbox doctor || true"), agent("coda-sandbox doctor || true")):
        assert any(l.split()[:2] == ["pass", "config"] for l in out.splitlines()), out

with subtest("gc unit runs coda-sandbox worktree gc in the checkout"):
    unit = machine.succeed(in_container + "systemctl cat coda-worktree-gc.service")
    assert "coda-sandbox worktree gc" in unit, unit
    agent(
        "git init -q /agents/coda/src/coda-rust"
        " && git -C /agents/coda/src/coda-rust -c user.name=t -c user.email=t@t"
        " commit -q --allow-empty -m init"
        " && mkdir -p /agents/coda/src/coda-rust/.worktrees/live"
        " /agents/coda/build/wt/live /agents/coda/build/wt/orphan"
        " && touch /agents/coda/build/wt/live/kept /agents/coda/build/wt/orphan/gone"
    )
    machine.succeed(in_container + "systemctl start coda-worktree-gc.service")
    result = machine.succeed(
        in_container + "systemctl show -P Result coda-worktree-gc.service"
    ).strip()
    assert result == "success", result
    journal = machine.succeed(in_container + "journalctl -u coda-worktree-gc.service -o cat")
    assert "Removing /agents/coda/build/wt/orphan" in journal, journal
    machine.fail("test -e /agents/coda/build/wt/orphan")
    machine.succeed("test -e /agents/coda/build/wt/live/kept")

with subtest("arp_ignore: the container sets its own, whatever the host held"):
    # Raise the host's to 2, as the Mullvad daemon does, and restart the
    # container: it copies 2 at start, then its own systemd-sysctl sets 1.
    machine.succeed("sysctl -w net.ipv4.conf.all.arp_ignore=2")
    machine.succeed("systemctl restart container@coda-agent.service")
    machine.wait_until_succeeds(
        in_container + "systemctl is-active systemd-sysctl.service", timeout=60
    )
    inside = machine.succeed(
        in_container + "cat /proc/sys/net/ipv4/conf/all/arp_ignore"
    ).strip()
    assert inside == "1", inside
    machine.succeed("sysctl -w net.ipv4.conf.all.arp_ignore=1")
