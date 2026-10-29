# Sign-in data promises

Promises from [physician.md A1](../../../intent/physician.md#a1-sign-in-and-access) about
records rather than screens. The chart can't hold them as finite states. Each names
the Rust owner that must prove it, and stays unproven until that proof exists.

| Promise | Spec | Owner | Proof |
| --- | --- | --- | --- |
| An address with no account gets no email, and the answer looks the same either way. | A1.2 | `crates/physician-auth` | Unproven |
| A sign-in link is stored only as a hash, expires, and signs in at most once. | A1.2, A1.5 | `crates/physician-auth` (`token_hash`, pending-token consume) | Unproven |
| The emailed link carries no patient. | A1.3 | `crates/physician-auth` | Unproven |
| After signing back in, drafts and the open reader are intact, and Coda's running work never notices the expiry. | A1.3 | `crates/physician-sample` (drafts and opened state are server-held, A1.14) | Unproven |
| Drafts and seen state stay per physician when another physician's link signs in. | A1.4 | `crates/physician-sample` | Unproven |
| Accounts are invite-only; there is no public sign-up. | A1.5 | `crates/physician-auth` (`seed_account`, invitations) | Unproven |
