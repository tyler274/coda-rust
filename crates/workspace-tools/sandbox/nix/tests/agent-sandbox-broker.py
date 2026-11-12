# The token broker half of the agent-sandbox check, appended to the test
# script in agent-sandbox.nix, which defines github_token, doppler_token,
# keyring and host_users_gid. The broker is luluco's lingering user service; its keyring
# is the stub secret-tool, whose items are files under `keyring`.

as_luluco = "su -l luluco -c "
user_env = "export XDG_RUNTIME_DIR=/run/user/$(id -u); "


def luluco(cmd):
    return machine.succeed(as_luluco + json.dumps(user_env + cmd))


# The broker's stdout, tagged with its program name; host commands run here
# print to the terminal, not the journal.
broker_journal_cmd = "journalctl --no-pager -o cat -t coda-sandbox"


def broker_journal():
    return machine.succeed(broker_journal_cmd)


with subtest("broker: a lingering user service listening on the agent socket"):
    machine.wait_for_unit("coda-sandbox-broker.service", "luluco")
    machine.wait_for_file("/run/coda-sandbox/agent/broker.sock")
    mode = machine.succeed("stat -c '%a %U %g' /run/coda-sandbox/agent").strip()
    assert mode == f"750 luluco {host_users_gid}", mode

with subtest("broker store reads stdin and reloads the running broker"):
    out = luluco(f"printf %s {github_token} | coda-sandbox broker store github-agent")
    assert "running broker" in out, out
    luluco(f"printf '%s\\n' {doppler_token} | coda-sandbox broker store doppler")
    stored = machine.succeed(f"cat {keyring}/github-agent").strip()
    assert stored == github_token, stored

with subtest("the owner sees the token sources, never a token"):
    status = luluco("coda-sandbox status")
    assert "github-agent: keyring; doppler: keyring" in status, status
    doctor = luluco("coda-sandbox doctor || true")
    assert "token broker" in doctor and github_token not in doctor, doctor

with subtest("git authenticates through the credential helper"):
    cred = agent("printf 'protocol=https\\nhost=github.com\\n\\n' | git credential fill")
    assert f"password={github_token}" in cred, cred
    assert "username=x-access-token" in cred, cred
    # Other hosts get nothing from the helper, and git does not prompt.
    agent_fails("printf 'protocol=https\\nhost=example.com\\n\\n' | git credential fill")

with subtest("gh and doppler wrappers get their tokens per process"):
    assert agent("gh auth token").strip() == github_token
    assert agent("coda-sandbox token doppler").strip() == doppler_token
    agent("doppler --version")
    agent_fails("printenv GH_TOKEN")
    agent_fails("printenv DOPPLER_TOKEN")

with subtest("only the container's agent gets a secret"):
    err = machine.fail("coda-sandbox token gh 2>&1")
    assert "may not ask" in err, err
    err = machine.fail(as_luluco + json.dumps("coda-sandbox token gh 2>&1"))
    assert "may not ask" in err, err
    # Container root is host uid uidBase, outside the socket directory's
    # group, so as itself it cannot reach the socket.
    err = machine.fail("nixos-container run coda-agent -- coda-sandbox token gh 2>&1")
    assert "Permission denied" in err, err
    # Another container user in `users` reaches it, and the peer check
    # refuses it. Container root can take any uid in its namespace, the
    # agent's included, so the container as a whole is the principal.
    other_user = "setpriv --reuid=1001 --regid=100 --clear-groups "
    err = machine.fail(
        "nixos-container run coda-agent -- " + other_user + "coda-sandbox token gh 2>&1"
    )
    assert "may not ask" in err, err

with subtest("a locked keyring gives git no answer and a typed reason"):
    machine.succeed(f"touch {keyring}/locked")
    # What ExecReload sends; this VM's lingering user has no user bus for
    # `systemctl --user reload`.
    machine.succeed("pkill -HUP -u luluco -f 'coda-sandbox broker serve'")
    machine.wait_until_succeeds(broker_journal_cmd + " | grep -q 'broker reload by SIGHUP'", timeout=10)
    agent_fails("printf 'protocol=https\\nhost=github.com\\n\\n' | git credential fill")
    err = agent("coda-sandbox token gh 2>&1 || true")
    assert "locked" in err, err
    machine.succeed(f"rm {keyring}/locked")
    assert agent("gh auth token").strip() == github_token

with subtest("the audit log records every request and no secret"):
    journal = broker_journal()
    assert "granted" in journal and "wrong_peer" in journal, journal
    assert github_token not in journal and doppler_token not in journal, journal

with subtest("the broker cannot dump core"):
    pid = machine.succeed("pgrep -u luluco -f 'coda-sandbox broker serve'").strip()
    limits = machine.succeed(f"cat /proc/{pid}/limits")
    assert any(l.startswith("Max core file size") and l.split()[4:6] == ["0", "0"]
               for l in limits.splitlines()), limits
    # Not dumpable: the kernel hands /proc/<pid>/mem and ptrace to root
    # only, even against the broker's own user.
    owner = machine.succeed(f"stat -c %U /proc/{pid}/mem").strip()
    assert owner == "root", owner

with subtest("polkit: the user restarts the container, never a transient root unit"):
    # The CI is off here, so no coda-ci unit exists and the rule names none;
    # the request falls to polkit's admin default, which a caller without a
    # prompt cannot pass. The container's name is in the rule, and the rule
    # refuses a transient unit under it.
    machine.fail("systemctl cat coda-ci.service")
    # Whether polkit wanted a prompt tells the two apart.
    for unit, wants_prompt in (
        ("coda-ci.service", True),
        ("container@coda-agent.service", False),
    ):
        marker = "/tmp/transient-" + unit
        err = machine.fail(as_luluco + json.dumps(
            f"systemd-run --no-ask-password --unit={unit} --wait touch {marker} 2>&1"
        ))
        assert "Access denied" in err, (unit, err)
        assert ("interactive authentication" in err) == wants_prompt, (unit, err)
        machine.fail(f"test -e '{marker}'")
    machine.fail(as_luluco + json.dumps(
        "systemctl --no-ask-password reload-or-restart container@coda-agent.service"
    ))
    luluco("systemctl --no-ask-password restart container@coda-agent.service")
    machine.wait_for_unit("container@coda-agent.service")
    machine.wait_until_succeeds(in_agent + json.dumps("true"), timeout=60)

with subtest("a host without the CI has no CI socket and no CI token"):
    machine.fail("test -e /run/coda-sandbox/ci")
    status = luluco("coda-sandbox status")
    assert "github-ci" not in status, status
