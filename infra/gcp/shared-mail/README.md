# Production shared mail

This root owns Coda's production SMTP adapter in `coda-health-prod`, `us-east1`.
It uses the existing [`gmail` transport](/crates/gmail) and the same reviewed
image as staging, copied by digest to its own registry. It owns only its mail
registry, network, static egress, runtime identity, Cloud Run service and named
invokers. The existing project, enabled APIs and state bucket keep their owners.
The state prefix is `coda-rust/shared-mail/production`; no product state is read
or changed. Rust staging stays in [rust-environments](/infra/gcp/rust-environments).

The runtime identity has no product-data roles or secrets. Cloud Run IAM admits
the explicitly configured callers. All outbound traffic uses the one reserved
NAT address; Workspace requires that exact address, domain-owned senders and TLS.
The service sends as `signin@codahealth.ai`. Products own content, recipients,
sign-in codes and send recovery. There is no queue or automatic retry here.

## Prepare and deploy

Production changes require the operator's approval of the saved plan. This is
the mail-only exception to the workspace's otherwise staging-only apply scope.
Preparation does not authorize a Gym production deployment or an email.

1. Initialize this root with `tofu init -input=false`, then save and inspect
   `tofu plan -input=false -out=prepare.tfplan` for the first deployment only.
   Without the service inputs, `image` defaults to null; this prepares
   the registry, network and static egress only. Apply the saved plan after approval.
2. Read `tofu output -raw smtp_egress_ip`. Add that exact `/32` to the existing
   Google Workspace SMTP relay policy, requiring TLS and domain-owned senders.
   Do not remove staging's address. NAT and the reserved IP incur charges even
   before the service handles mail.
3. Build `crates/gmail/Dockerfile` once from the reviewed workspace root for
   `linux/amd64`. Run the image locally, confirm non-root execution, readiness
   and invalid-recipient refusal. Publish it to
   `us-east1-docker.pkg.dev/coda-rust-staging/apps/mail-relay` and verify its digest.
   Copy that digest to `us-east1-docker.pkg.dev/coda-health-prod/shared-mail/mail-relay`
   with a digest-preserving copy of all manifests (`skopeo copy --all --preserve-digests`),
   then verify that both remote digests equal the selected local image digest.
   Do not rebuild for production. No secret or local settings file belongs in the build context.
4. The versioned `env/production.tfvars` owns the selected production image and callers;
   staging's `rust-environments/env/staging.tfvars` owns its selection. Both select the
   same image digest, with only the registry path differing. Update these non-secret
   inputs together for each reviewed release, and publish and verify the selected bytes
   before applying. Use `tofu plan -input=false -var-file=env/production.tfvars -out=service.tfplan`
   for the service and every subsequent plan. Omitting the file proposes removing the
   service. Review and apply the saved plan only after approval; rollback selects the
   previous digest from version history through the same procedure. A source pin is
   desired configuration, not deployment or delivery evidence.
5. Read back the service image, runtime identity, IAM, network and static egress.
   `/readyz` proves process readiness only. Verify unauthenticated denial and
   authenticated access from the actual product runtime. Mailbox receipt and
   sign-in completion require an explicitly approved test message and recipient.

Gym owns its environment settings, site deployment, end-to-end mailbox and send
record proof, and retirement of its old Gmail credential in its
[SMTP cutover procedure](https://github.com/Coda-Health/coda-gym/blob/main/authoring/web/infra/README.md#shared-smtp-delivery-and-cutover).
Keep the old working site and credential until that proof succeeds. Returning
to the previous image uses the same reviewed-plan procedure; an uncertain send
is settled by the product before another attempt.

## Offline checks

`tofu fmt -check -recursive`, `tofu init -backend=false -input=false`,
`tofu validate` and `tofu test` check this root without cloud access. The mock
plans guard the production registry, named invoker and static-egress boundary;
they do not establish SMTP delivery or Workspace configuration.
