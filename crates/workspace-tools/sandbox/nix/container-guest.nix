# NixOS configuration inside the coda-agent container (on top of what
# nspawn.nix sets for every sandbox).
{
  repo,
  package,
  swarmPackage,
}:
{ lib, pkgs, ... }:
let
  coda-sandbox = lib.getExe' package "coda-sandbox";
  # gh, Doppler and the coding agents read their tokens from the
  # environment, so each wrapper asks the host's token broker for one per
  # run and hands it to that process alone: never to the calling shell, and
  # never to a file. `env` and `args` keep the agents from passing the key
  # on to the commands they run.
  wrap =
    {
      name,
      real,
      want,
      var,
      env ? [ ],
      args ? [ ],
    }:
    pkgs.writeShellApplication {
      inherit name;
      text = ''
        token="$(${coda-sandbox} token ${want})" || exit 1
        ${var}="$token" ${lib.concatStringsSep " " env} exec ${lib.getExe real} ${lib.escapeShellArgs args} "$@"
      '';
    };
in
{
  # claude-code is unfree, and the container evaluates its own nixpkgs.
  nixpkgs.config.allowUnfreePredicate = p: lib.getName p == "claude-code";

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

  # Gemini CLI strips variables named like secrets from the commands it
  # runs only with redaction on; system settings override the user's.
  environment.etc."gemini-cli/settings.json".text = builtins.toJSON {
    security.environmentVariableRedaction.enabled = true;
    security.auth.selectedType = "gemini-api-key";
  };

  environment.systemPackages =
    with pkgs;
    [
      (wrap {
        name = "gh";
        real = gh;
        want = "gh";
        var = "GH_TOKEN";
      })
      (wrap {
        name = "doppler";
        real = doppler;
        want = "doppler";
        var = "DOPPLER_TOKEN";
      })
      # Claude Code keeps the key for its own API calls and strips it from
      # its Bash commands, hooks and MCP servers.
      (wrap {
        name = "claude";
        real = claude-code;
        want = "anthropic";
        var = "ANTHROPIC_API_KEY";
        env = [ "CLAUDE_CODE_SUBPROCESS_ENV_SCRUB=1" ];
      })
      # Codex reads OPENAI_API_KEY only through a provider's env_key, and
      # with the default excludes back on it leaves *KEY*, *SECRET* and
      # *TOKEN* variables out of the commands it runs.
      (wrap {
        name = "codex";
        # nixpkgs' cargo hook forces system zstd and AWS-LC, which codex
        # 0.167 does not find; its bundled copies build. Drop once
        # nixpkgs' codex builds as is.
        real = codex.overrideAttrs { dontCargoUseSystemLibs = true; };
        want = "openai";
        var = "OPENAI_API_KEY";
        args = [
          "-c"
          ''model_providers.coda-openai={name="OpenAI",base_url="https://api.openai.com/v1",env_key="OPENAI_API_KEY",wire_api="responses"}''
          "-c"
          "model_provider=coda-openai"
          "-c"
          "shell_environment_policy.ignore_default_excludes=false"
        ];
      })
      (wrap {
        name = "gemini";
        real = gemini-cli;
        want = "gemini";
        var = "GEMINI_API_KEY";
      })
      gnumake
      ripgrep
      fd
      rsync
      file
      which
    ]
    ++ lib.optional (swarmPackage != null) swarmPackage;

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
