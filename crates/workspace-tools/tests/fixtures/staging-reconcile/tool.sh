#!/bin/sh
set -eu
base=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
name=${0##*/}
[ "${TF_WORKSPACE:-}" = default ]
[ -z "${TF_CLI_ARGS_plan:-}" ]
[ -z "${TF_LOG:-}" ]
[ -z "${TF_LOG_PATH:-}" ]
[ -z "${TF_VAR_environment:-}" ]
printf '%s argv=%s env=%s\n' "$name" "$*" "${TF_VAR_database_passwords:-none}" >> "$base/calls.log"
case "$name" in
  doppler)
    case "$8" in
      stg_physician_runtime) echo pr-synthetic-secret ;;
      *) echo "${8}-synthetic-secret" ;;
    esac
    ;;
  tofu)
    case "$2" in
      init) ;;
      plan)
        for a; do
          case "$a" in -out=*) printf bytes > "${a#-out=}" ;; esac
        done
        case "$*" in *-out=*) exit 2 ;; esac
        ;;
      show)
        if [ "$3" = -json ]; then cat "$base/show.json"; else echo review; fi
        ;;
      apply) ;;
      *) exit 9 ;;
    esac
    ;;
  gcloud) cat "$base/service.json" ;;
  *) exit 9 ;;
esac
