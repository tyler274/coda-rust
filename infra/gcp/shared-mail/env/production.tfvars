# Selected sender-aware image qualified on staging.
# Publish and verify this exact digest before applying; this file is desired
# configuration, not evidence that the image or service has been deployed.
image    = "us-east1-docker.pkg.dev/coda-health-prod/shared-mail/mail-relay@sha256:4179edf841befcfa33b5dcdb0e1efe0bf20d1dce184fe9359efcb9eef71c6ea4"
invokers = ["serviceAccount:workspace-web@coda-gym.iam.gserviceaccount.com"]
