#!/usr/bin/env bash
# Fast crate-scoped formatting, Clippy, and unit test runner for local iteration.
# Defaults to modified crates (working tree, or HEAD~1..HEAD when clean).
# Pass --base <ref>, --all, --pre-commit (staged index only, skipping cargo test),
# --ci (full local CI pass against an isolated disposable Postgres cluster),
# --postgres (disposable Postgres suites only, combinable with --ci/--browser), or
# --browser (Chromium qualification, or combined with --ci/--postgres).
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"

orig_args=("$@")
mode="local"
base_ref=""
run_ci=0
run_postgres=0
run_browser=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    --pre-commit) mode="pre-commit" ;;
    --all) mode="all" ;;
    --ci) run_ci=1 ;;
    --postgres) run_postgres=1 ;;
    --browser) run_browser=1 ;;
    --base)
      shift
      base_ref="${1:?--base requires a git ref}"
      mode="base"
      ;;
    -h|--help)
      sed -n '2,7p' "$0"
      exit 0
      ;;
    *)
      echo "dev-check.sh: unknown argument '$1'" >&2
      exit 2
      ;;
  esac
  shift
done

if [ "$run_ci" = 1 ] || [ "$run_postgres" = 1 ] || [ "$run_browser" = 1 ]; then
  if [ "$mode" != "local" ]; then
    echo "dev-check.sh: --ci, --postgres, and --browser cannot be combined with --$mode" >&2
    exit 2
  fi
  mode="ci"
fi

if [ "${CODA_DEV_SHELL:-}" != 1 ]; then
  if [ -z "${CODA_DEV_CHECK_REEXEC:-}" ] && command -v nix >/dev/null 2>&1; then
    export CODA_DEV_CHECK_REEXEC=1
    exec nix develop "${root}#agent" --command bash "$0" ${orig_args[@]+"${orig_args[@]}"}
  fi
  if [ "$mode" = "ci" ]; then
    echo "dev-check.sh: run --ci/--postgres/--browser inside 'nix develop .#agent' (CODA_DEV_SHELL unset)" >&2
    exit 1
  fi
fi

tofu_fmt_roots=(
  infra/gcp/nix-cache
  infra/gcp/shared-mail
  infra/gcp/public-site
  infra/gcp/observability
  infra/gcp/physician-runtime
  infra/gcp/rust-environments
)

run_tofu_fmt_checks() {
  local root_dir
  for root_dir in "${tofu_fmt_roots[@]}"; do
    tofu fmt -check -recursive "$root_dir"
  done
}

run_formatting_checks() {
  local include_tofu="${1:-0}"
  cargo fmt --all --check
  python3 scripts/check-sql.py
  if [ "$include_tofu" = 1 ]; then
    run_tofu_fmt_checks
  fi
}

wasm_clippy() {
  cargo clippy -p physician-app -p duet-app -p mobile -p ui-gallery -p extension-popup \
    --target wasm32-unknown-unknown \
    --features physician-app/web,duet-app/web,mobile/web,ui-gallery/web,extension-popup/web \
    --quiet -- -D warnings
}

stage_names=()
stage_statuses=()
stage_durations=()
overall_rc=0
total_start_secs="$SECONDS"
pg_dir=""
pg_base=""

format_duration() {
  local secs="$1"
  if [ "$secs" -ge 60 ]; then
    printf '%dm%02ds' $((secs / 60)) $((secs % 60))
  else
    printf '%ds' "$secs"
  fi
}

cleanup_ci() {
  local exit_rc=$?
  if [ -n "$pg_dir" ] && [ -f "$pg_dir/data/postmaster.pid" ]; then
    pg_ctl -D "$pg_dir/data" -m fast -w stop >/dev/null 2>&1 || \
      pg_ctl -D "$pg_dir/data" -m immediate -w stop >/dev/null 2>&1 || true
  fi
  if [ -n "$pg_dir" ]; then
    rm -rf "$pg_dir"
  fi
  if [ "${#stage_names[@]}" -gt 0 ]; then
    local total_elapsed=$((SECONDS - total_start_secs))
    printf '\n%-24s %-8s %s\n' 'Stage' 'Result' 'Duration'
    printf '%-24s %-8s %s\n' '------------------------' '------' '--------'
    local i
    for ((i = 0; i < ${#stage_names[@]}; i++)); do
      printf '%-24s %-8s %s\n' \
        "${stage_names[$i]}" \
        "${stage_statuses[$i]}" \
        "$(format_duration "${stage_durations[$i]}")"
    done
    printf '%-24s %-8s %s\n' '------------------------' '------' '--------'
    local overall_label="PASS"
    if [ "$overall_rc" -ne 0 ] || [ "$exit_rc" -ne 0 ]; then
      overall_label="FAIL"
    fi
    printf '%-24s %-8s %s\n' 'TOTAL' "$overall_label" "$(format_duration "$total_elapsed")"
  fi
}

run_stage() {
  local name="$1"
  shift
  printf '\n==> [dev-check] %s\n' "$name"
  local start_secs="$SECONDS"
  local rc=0
  set +e
  "$@"
  rc=$?
  set -e
  local elapsed=$((SECONDS - start_secs))
  stage_names+=("$name")
  stage_durations+=("$elapsed")
  if [ "$rc" -eq 0 ]; then
    stage_statuses+=("PASS")
    return 0
  fi
  stage_statuses+=("FAIL($rc)")
  overall_rc="$rc"
  return "$rc"
}

ensure_ci_python() {
  if python3 -c "import yaml" >/dev/null 2>&1; then
    printf '%s\n' "python3"
    return 0
  fi
  local venv="${CODA_CI_VENV:-/tmp/coda-ci-venv-$(id -un)}"
  if [ ! -x "$venv/bin/python" ] || ! "$venv/bin/python" -c "import yaml" >/dev/null 2>&1; then
    rm -rf "$venv"
    python3 -m venv "$venv"
    "$venv/bin/pip" install --quiet -r "$root/scripts/ci-requirements.txt"
  fi
  printf '%s\n' "$venv/bin/python"
}

pick_free_port() {
  python3 -c '
import socket
while True:
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        port = s.getsockname()[1]
    if port != 55489:
        print(port)
        break
'
}

start_disposable_postgres() {
  pg_dir=$(mktemp -d "${TMPDIR:-/tmp}/coda-pg.XXXXXX")
  pg_port=$(pick_free_port)
  initdb -U postgres -A trust -D "$pg_dir/data" >/dev/null &&
    pg_ctl -D "$pg_dir/data" -l "$pg_dir/postgres.log" \
      -o "-p ${pg_port} -h 127.0.0.1 -k ${pg_dir} -N 200" -w start >/dev/null || return $?
  pg_base="postgresql://postgres@127.0.0.1:${pg_port}"
  export MIGRATION_DATABASE_URL="${pg_base}/postgres"
  printf 'dev-check: disposable Postgres ready on 127.0.0.1:%s (%s)\n' "$pg_port" "$pg_dir"
}

run_postgres_suites() {
  (
    set -euo pipefail
    createdb --maintenance-db "$MIGRATION_DATABASE_URL" front_door_ci
    createdb --maintenance-db "$MIGRATION_DATABASE_URL" physician_ci
    unset DUET_TEST_DATABASE_URL PHYSICIAN_TEST_DATABASE_URL \
      FRONT_DOOR_TEST_DATABASE_URL CLINICAL_LIBRARY_TEST_DATABASE_URL
    export PHYSICIAN_DATABASE_URL="${pg_base}/physician_ci"
    export PHYSICIAN_RUNTIME_TEST_DATABASE_URL="${pg_base}/physician_ci"
    export FRONT_DOOR_DATABASE_URL="${pg_base}/front_door_ci"
    export DUET_IDENTITY_TEST_DATABASE_URL="${pg_base}/front_door_ci"
    export CODA_REQUIRE_POSTGRES_TESTS=1
    bash scripts/check-postgres.sh
    cargo test --locked -p physician-sample --test queue_trace -- --test-threads=1
  )
}

run_lightweight_checks() {
  (
    set -euo pipefail
    local ci_python
    ci_python=$(ensure_ci_python)
    "$ci_python" -m unittest discover -s scripts -p 'test_ci_*.py' -v
    "$ci_python" -m unittest discover -s scripts -p 'test_check_sql.py' -v
    "$ci_python" -m unittest discover -s scripts -p 'test_staging_images.py' -v
    "$ci_python" -m unittest discover -s scripts -p 'test_alert_qualification.py' -v
    "$ci_python" -m unittest discover -s scripts/nix-cache -p 'test_*.py' -v
    "$ci_python" scripts/check_engineering_skills.py
    python3 -m unittest discover -s .agents/skills/review-pr/scripts -p 'test_*.py' -v
    python3 -m unittest discover -s .agents/skills/capture-proof/scripts -p 'test_*.py' -v
    node --test crates/workspace-tools/tests/doppler-watch.test.cjs
    actionlint .github/workflows/*.yml
    lychee --offline --no-progress --include-fragments --root-dir "$root" '.agents/skills/**/*.md'
    bash scripts/check-book-links.sh
    node extension/sandbox/test_connector.mjs
  )
}

run_workspace_tests() {
  (
    set -euo pipefail
    if [ -n "$pg_base" ]; then
      export DUET_TEST_DATABASE_URL="${pg_base}/postgres"
      export PHYSICIAN_TEST_DATABASE_URL="${pg_base}/postgres"
      export FRONT_DOOR_TEST_DATABASE_URL="${pg_base}/postgres"
      export CLINICAL_LIBRARY_TEST_DATABASE_URL="${pg_base}/postgres"
      unset PHYSICIAN_DATABASE_URL PHYSICIAN_RUNTIME_TEST_DATABASE_URL \
        FRONT_DOOR_DATABASE_URL DUET_IDENTITY_TEST_DATABASE_URL \
        CODA_REQUIRE_POSTGRES_TESTS
    fi
    if [ ! -f "$root/submodules/coda-app/backend/app/services/health_text_encoder_worker.py" ] ||
       [ ! -f "$root/submodules/coda-agents/src/adapters/patientDiscovery/patient_text_encoder.py" ]; then
      unset DUET_DISCOVERY_ENCODER_MODEL CODA_PATIENT_ENCODER_MODEL
    fi
    cargo test --locked --workspace
  )
}

run_browser_qualification() {
  (
    set -euo pipefail
    createdb --maintenance-db "$MIGRATION_DATABASE_URL" physician_browser
    createdb --maintenance-db "$MIGRATION_DATABASE_URL" duet_browser
    unset DUET_TEST_DATABASE_URL PHYSICIAN_TEST_DATABASE_URL \
      FRONT_DOOR_TEST_DATABASE_URL CLINICAL_LIBRARY_TEST_DATABASE_URL \
      FRONT_DOOR_DATABASE_URL DUET_IDENTITY_TEST_DATABASE_URL
    export PHYSICIAN_DATABASE_URL="${pg_base}/physician_browser"
    export PHYSICIAN_RUNTIME_TEST_DATABASE_URL="${pg_base}/physician_browser"
    export DUET_TEST_DATABASE_URL="${pg_base}/duet_browser"
    export CODA_REQUIRE_POSTGRES_TESTS=1
    cargo test --locked -p physician-sample \
      --test site --test auth_browser --test sign_in_states --test shell_states \
      --test reply_revision --test sign_in_model \
      -- --ignored --test-threads=1
    cargo test --locked -p public-site --test site -- --ignored
    cargo test --locked -p mobile \
      --test browser --test duet_journey --test mode \
      -- --ignored --skip backend_process --test-threads=1
    cargo test --locked -p ui-gallery --test sections -- --ignored
    cargo test --locked -p extension-popup --test popup -- --ignored
  )
}

if [ "$mode" = "ci" ]; then
  trap cleanup_ci EXIT
  run_stage "fmt" run_formatting_checks 1
  if [ "$run_ci" = 1 ]; then
    run_stage "clippy-host" cargo clippy --workspace --all-targets --quiet -- -D warnings
    run_stage "clippy-wasm" wasm_clippy
    run_stage "boundaries" cargo test --locked -p boundaries
    run_stage "cargo-deny" cargo deny check
    run_stage "rustdoc" cargo doc --no-deps --workspace
    run_stage "lightweight" run_lightweight_checks
  fi
  run_stage "postgres-cluster" start_disposable_postgres
  if [ "$run_ci" = 1 ]; then
    run_stage "workspace-tests" run_workspace_tests
  fi
  if [ "$run_ci" = 1 ] || [ "$run_postgres" = 1 ]; then
    run_stage "postgres-suites" run_postgres_suites
  fi
  if [ "$run_browser" = 1 ]; then
    run_stage "browser" run_browser_qualification
  fi
  exit "$overall_rc"
fi

if [ "$mode" = "all" ]; then
  run_formatting_checks 1
  cargo clippy --workspace --all-targets --quiet -- -D warnings
  wasm_clippy
  run_workspace_tests
  exit 0
fi

run_formatting_checks 0

changed_files=""
case "$mode" in
  pre-commit)
    changed_files=$(git diff --cached --name-only --diff-filter=ACMR)
    ;;
  base)
    changed_files=$(git diff --name-only --diff-filter=ACMR "${base_ref}...HEAD" ; git status --porcelain=v1 | awk '{print $NF}')
    ;;
  local)
    changed_files=$(git status --porcelain=v1 | awk '{print $NF}')
    if [ -z "$changed_files" ]; then
      changed_files=$(git diff --name-only --diff-filter=ACMR HEAD~1..HEAD 2>/dev/null || true)
    fi
    ;;
esac

if [ -z "$changed_files" ]; then
  echo "dev-check: no changed files; formatting ok."
  exit 0
fi

needs_workspace=0
needs_wasm=0
needs_tofu_fmt=0
pkg_list=""

crate_package_name() {
  local manifest="$1/Cargo.toml"
  [ -f "$manifest" ] || return 1
  awk -F'"' '/^name[[:space:]]*=/{print $2; exit}' "$manifest"
}

while IFS= read -r path; do
  [ -n "$path" ] || continue
  case "$path" in
    Cargo.toml|Cargo.lock|clippy.toml|rust-toolchain.toml|.cargo/*|flake.nix|flake.lock|crates/workspace-tools/sandbox/*)
      needs_workspace=1
      ;;
    infra/gcp/*)
      needs_tofu_fmt=1
      ;;
    crates/*)
      seg="${path#crates/}"
      crate_dir="crates/${seg%%/*}"
      if pkg=$(crate_package_name "$crate_dir"); then
        case " $pkg_list " in
          *" $pkg "*) ;;
          *) pkg_list="$pkg_list $pkg" ;;
        esac
        case "$pkg" in
          ui|physician-app|duet-app|mobile|ui-gallery|extension-popup|physician-client|duet-client|coda-http|physician-wire|duet_wire)
            needs_wasm=1
            ;;
        esac
      else
        needs_workspace=1
      fi
      ;;
  esac
done <<<"$changed_files"

if [ "$needs_tofu_fmt" = 1 ]; then
  run_tofu_fmt_checks
fi

pkg_count=$(wc -w <<<"$pkg_list")
if [ "$needs_workspace" = 1 ] || [ "$pkg_count" -gt 8 ]; then
  cargo clippy --workspace --all-targets --quiet -- -D warnings
  wasm_clippy
  if [ "$mode" != "pre-commit" ]; then
    run_workspace_tests
  fi
  exit 0
fi

if [ "$pkg_count" -eq 0 ]; then
  echo "dev-check: no Rust crates changed; formatting ok."
  exit 0
fi

pkg_args=()
for pkg in $pkg_list; do
  pkg_args+=(-p "$pkg")
done

echo "dev-check: checking crates:${pkg_list}"
cargo clippy "${pkg_args[@]}" --all-targets --quiet -- -D warnings
if [ "$needs_wasm" = 1 ]; then
  wasm_clippy
fi
if [ "$mode" != "pre-commit" ]; then
  cargo test --locked "${pkg_args[@]}"
fi
