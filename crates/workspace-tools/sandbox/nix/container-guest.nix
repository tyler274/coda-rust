# NixOS configuration inside the coda-agent container (on top of what
# nspawn.nix sets for every sandbox).
{
  repo,
  package,
  modelKeys,
}:
{ lib, pkgs, ... }:
let
  coda-sandbox = lib.getExe' package "coda-sandbox";
  # Claude Code and Codex come from the repo's dev shell, not from here, so
  # the key reaches them through their system-wide config rather than a
  # wrapper on PATH: each runs the broker's `token` command per session.
  toml = pkgs.formats.toml { };
  # gh and Doppler read their tokens from the environment and have no
  # credential hook, so each wrapper asks the host's token broker for one
  # per run and hands it to that process alone.
  wrap =
    name: real: want: var:
    pkgs.writeShellApplication {
      inherit name;
      text = ''
        token="$(${coda-sandbox} token ${want})" || exit 1
        ${var}="$token" exec ${lib.getExe real} "$@"
      '';
    };
in
{
  programs.direnv = {
    enable = true;
    nix-direnv.enable = true;
  };

  programs.git = {
    enable = true;
    config = {
      # The agent has no SSH key; GitHub is reached over HTTPS with the
      # scoped token, including remotes written as git@github.com:.
      url."https://github.com/".insteadOf = [
        "git@github.com:"
        "ssh://git@github.com/"
      ];
      # Asks the token broker on every fetch and push; the token never lands
      # in a file or a git config.
      credential."https://github.com".helper = "${coda-sandbox} credential";
      init.defaultBranch = "main";
    };
  };

  # With no answer from the broker, git fails instead of waiting at a prompt.
  environment.variables.GIT_TERMINAL_PROMPT = "0";

  # Managed settings outrank the agent's own, so the broker's key wins over
  # a login while this is on.
  environment.etc."claude-code/managed-settings.json" = lib.mkIf modelKeys.anthropic {
    text = builtins.toJSON { apiKeyHelper = "${coda-sandbox} token anthropic"; };
  };
  # The system layer of Codex's config; the agent's ~/.codex/config.toml
  # can still choose another provider.
  environment.etc."codex/config.toml" = lib.mkIf modelKeys.openai {
    source = toml.generate "codex-config.toml" {
      model_provider = "openai-broker";
      model_providers.openai-broker = {
        name = "OpenAI (coda-sandbox broker key)";
        base_url = "https://api.openai.com/v1";
        wire_api = "responses";
        auth = {
          command = coda-sandbox;
          args = [
            "token"
            "openai"
          ];
        };
      };
    };
  };

  environment.systemPackages = with pkgs; [
    (wrap "gh" gh "gh" "GH_TOKEN")
    (wrap "doppler" doppler "doppler" "DOPPLER_TOKEN")
    gnumake
    # Antigravity's remote installer downloads its server with wget only.
    wget
    ripgrep
    fd
    rsync
    file
    which
  ];

  # Returns the disk of idle worktree builds. coda-sandbox reads the build
  # root and the checkout from /etc/coda-sandbox/config.toml; until the
  # checkout is cloned there is nothing to collect.
  systemd.services.coda-worktree-gc = {
    description = "Reset idle coda-rust worktree builds to the seed";
    unitConfig.ConditionPathExists = "${repo}/.git";
    path = with pkgs; [
      coreutils
      git
    ];
    serviceConfig = {
      Type = "oneshot";
      User = "agent";
      Group = "users";
      WorkingDirectory = repo;
      ExecStart = "${lib.getExe' package "coda-sandbox"} worktree gc";
    };
  };
  systemd.timers.coda-worktree-gc = {
    wantedBy = [ "timers.target" ];
    timerConfig = {
      OnCalendar = "daily";
      Persistent = true;
      RandomizedDelaySec = "1h";
    };
  };
}
