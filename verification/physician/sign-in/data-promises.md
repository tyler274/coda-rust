# Sign-in data promises

Promises about records rather than screens, which the chart can't hold as finite states. Each names the Rust owner that keeps it, and stays unproven until a proof or replay shows it.

| Promise | Owner | Proof |
| --- | --- | --- |
| An address with no account gets no email, and the answer looks the same either way. | `crates/physician-auth` | Unproven |
| A sign-in link expires and signs in at most once. | `crates/physician-auth` | Unproven |
| The emailed link carries no patient. | `crates/physician-auth` | Unproven |
| After signing back in, drafts and the open reader are intact, and Coda's running work never notices the expiry. | `crates/physician-sample` | Unproven |
| Drafts and seen state stay with each physician when another physician signs in. | `crates/physician-sample` | Unproven |
| Accounts are invite-only, with no public sign-up. | `crates/physician-auth` | Unproven |
