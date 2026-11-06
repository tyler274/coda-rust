#!/usr/bin/env python3
"""Release-side checks that `deploy-staging` changes only the product image.

Infrastructure (infra/gcp/rust-environments) owns every Cloud Run service
field except the product container's image. With observability on, each
service runs two containers: the product, named after the service, and the
`otel-collector` sidecar. This script reads `gcloud run services describe
--format json` documents and never calls a cloud API itself.

    expected TFVARS             print `on` when the committed tfvars turn the
                                sidecar on, else `off`
    target SERVICE_JSON NAME EXPECTED
                                before rollout: check the running shape and
                                print the product container's name for
                                `gcloud run services update --container`
                                (empty for the single unnamed container)
    verify BEFORE AFTER NAME IMAGE
                                after rollout: the product serves IMAGE, the
                                shape is unchanged and still well formed, and
                                no infrastructure-owned template field moved

A service that runs the sidecar must carry the whole telemetry contract, or
`target` fails before anything is migrated or rolled out. A service whose
shape differs from the committed tfvars (the reviewed rust-environments plan
is not applied yet) is released as it stands with a workflow warning: the
release never adds, removes or reconfigures containers itself.
"""
from copy import deepcopy
import json
from pathlib import Path
import re
import sys

SIDECAR = "otel-collector"
# Set on the product container by observability-cluster.tf (DD_ENV and
# observability_product_env). None means any nonempty value.
PRODUCT_ENV = {
    "DD_ENV": None,
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://localhost:4318",
    "CODA_LOG_FORMAT": "json",
    "CODA_TELEMETRY_CONTENT": "none",
    "CODA_TELEMETRY_BACKEND": None,
}
SIDECAR_ENV = ("OTEL_SIDECAR_CONFIG", "OTEL_GATEWAY_ENDPOINT", "OTEL_GATEWAY_AUDIENCE")
# Written by every gcloud update; not infrastructure configuration.
CLIENT_ANNOTATIONS = ("run.googleapis.com/client-name", "run.googleapis.com/client-version")
CLIENT_LABELS = ("client.knative.dev/nonce",)


class ShapeError(Exception):
    pass


def expected(tfvars_text: str) -> str:
    def flag(name):
        match = re.search(rf"(?m)^\s*{name}\s*=\s*(true|false)\s*(#.*)?$", tfvars_text)
        return bool(match) and match.group(1) == "true"

    # observability-cluster.tf: local.observability needs both gates.
    return "on" if flag("observability_enabled") and flag("observability_cluster_enabled") else "off"


def containers(service: dict) -> list:
    return service.get("spec", {}).get("template", {}).get("spec", {}).get("containers", [])


def env(container: dict) -> dict:
    return {e["name"]: e.get("value") for e in container.get("env", [])}


def observed(service: dict) -> str:
    return "on" if any(c.get("name") == SIDECAR for c in containers(service)) else "off"


def product(service: dict, name: str) -> dict:
    found = containers(service)
    named = [c for c in found if c.get("name") == name]
    if len(named) == 1:
        return named[0]
    if len(found) == 1 and not found[0].get("name"):
        return found[0]
    raise ShapeError(f"{name}: no product container named {name!r} among {[c.get('name') for c in found]}")


def check_shape(service: dict, name: str) -> None:
    """The running shape is one the infrastructure root produces."""
    found = containers(service)
    names = [c.get("name") for c in found]
    if observed(service) == "off":
        # The single container stays unnamed while the gate is off.
        if len(found) != 1 or found[0].get("name"):
            raise ShapeError(f"{name}: expected one unnamed product container, found {names}")
        return
    if sorted(names, key=str) != sorted([name, SIDECAR]):
        raise ShapeError(f"{name}: expected containers [{name!r}, {SIDECAR!r}], found {names}")
    variables = env(product(service, name))
    for key, value in PRODUCT_ENV.items():
        if not variables.get(key) or (value is not None and variables[key] != value):
            raise ShapeError(f"{name}: product container lacks {key}" + (f"={value}" if value else ""))
    sidecar = next(c for c in found if c.get("name") == SIDECAR)
    missing = [key for key in SIDECAR_ENV if not env(sidecar).get(key)]
    if missing:
        raise ShapeError(f"{name}: {SIDECAR} lacks {', '.join(missing)}")
    if sidecar.get("ports"):
        raise ShapeError(f"{name}: {SIDECAR} must not take the ingress port")


def target(service: dict, name: str, expected_mode: str) -> tuple[str, str | None]:
    """Product container name and an optional warning for the workflow."""
    if expected_mode not in ("on", "off"):
        raise ShapeError(f"unknown expected observability mode {expected_mode!r}")
    check_shape(service, name)
    warning = None
    live = observed(service)
    if live != expected_mode:
        warning = (
            f"{name}: the committed tfvars turn observability {expected_mode}, but the service runs it {live}. "
            "Releasing the image only; apply the reviewed infra/gcp/rust-environments plan to reconcile."
        )
    return product(service, name).get("name") or "", warning


def infrastructure_view(service: dict, name: str) -> dict:
    """The revision template without the release-owned image and client metadata."""
    template = deepcopy(service.get("spec", {}).get("template", {}))
    metadata = template.setdefault("metadata", {})
    for key in CLIENT_ANNOTATIONS:
        metadata.get("annotations", {}).pop(key, None)
    for key in CLIENT_LABELS:
        metadata.get("labels", {}).pop(key, None)
    metadata.pop("name", None)
    product({"spec": {"template": template}}, name).pop("image", None)
    return template


def verify(before: dict, after: dict, name: str, image: str) -> None:
    check_shape(after, name)
    if observed(after) != observed(before):
        raise ShapeError(f"{name}: the release changed the container set")
    served = product(after, name).get("image")
    if served != image:
        raise ShapeError(f"{name} serves {served}, expected {image}")
    if infrastructure_view(before, name) != infrastructure_view(after, name):
        raise ShapeError(f"{name}: the release changed infrastructure-owned template fields, not only the product image")


def load(path: str) -> dict:
    return json.loads(Path(path).read_text())


def main(argv=None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    try:
        if args[:1] == ["expected"] and len(args) == 2:
            print(expected(Path(args[1]).read_text()))
        elif args[:1] == ["target"] and len(args) == 4:
            container, warning = target(load(args[1]), args[2], args[3])
            if warning:
                print(f"::warning::{warning}", file=sys.stderr)
            print(container)
        elif args[:1] == ["verify"] and len(args) == 5:
            verify(load(args[1]), load(args[2]), args[3], args[4])
        else:
            print(__doc__, file=sys.stderr)
            return 2
    except ShapeError as error:
        print(f"::error::{error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
