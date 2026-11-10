//! Reading mailbox changes from `users.history.list` with an opaque cursor.
//!
//! The cursor rules follow the Coda Agents Gmail correspondence adapter:
//!
//! - There is no cursor until [`History::first_cursor`] reads `getProfile` and
//!   starts from the mailbox's current `historyId`.
//! - A [`Cursor`] carries the start history id and, mid-listing, a page token.
//!   It moves to Gmail's returned `historyId` only on the last page
//!   (`nextPageToken` absent). Until then it keeps the start and takes the next
//!   page token, so a page that is processed but not committed is read again.
//! - A 404 for a stored cursor is [`ListError::CursorExpired`]: Gmail no longer
//!   has history that old, and the product must resynchronize.
//!
//! This reader does not send and stores nothing. The durable cursor, the seen
//! messages and the effects belong to the product's intake.
//! See <https://developers.google.com/gmail/api/reference/rest/v1/users.history/list>.
use serde::{Deserialize, Serialize};

use crate::{Credentials, Endpoints, Error, Tokens, http_client};

/// Least privilege for reading history. Used by service-account assertions;
/// a refresh token carries the scopes it was granted.
const GMAIL_READONLY_SCOPE: &str = "https://www.googleapis.com/auth/gmail.readonly";

/// `^\d{1,40}$`, the shape Gmail history ids take.
fn history_id_shape(value: &str) -> bool {
    (1..=40).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit())
}

/// Where a mailbox's changes are read from next. Opaque outside this module;
/// a product stores [`encode`](Self::encode) and restores with
/// [`decode`](Self::decode).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    start: String,
    page: Option<String>,
}

impl Cursor {
    /// The stored form.
    pub fn encode(&self) -> String {
        serde_json::to_string(self).expect("cursor json")
    }

    /// Restores a stored cursor; refuses anything this module did not write.
    pub fn decode(stored: &str) -> Result<Self, ListError> {
        let cursor: Self = serde_json::from_str(stored).map_err(|_| ListError::InvalidCursor)?;
        if !history_id_shape(&cursor.start) || cursor.page.as_deref() == Some("") {
            return Err(ListError::InvalidCursor);
        }
        Ok(cursor)
    }
}

/// One page of changes and the cursor to store once they are applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    /// Message ids added on this page, in history order. A message can appear
    /// again on a later page or a reread; deduplication is the caller's.
    pub messages_added: Vec<String>,
    /// Message ids deleted on this page, in history order.
    pub messages_deleted: Vec<String>,
    /// Store this only after the page's effects are durable.
    pub next: Cursor,
    /// Another page follows from `next`.
    pub has_more: bool,
}

/// Why a history read failed.
#[derive(Debug, thiserror::Error)]
pub enum ListError {
    /// Gmail answered 404 for the stored cursor: resynchronize.
    #[error("gmail history cursor expired")]
    CursorExpired,
    /// The stored cursor is not one this module wrote.
    #[error("gmail history cursor is invalid")]
    InvalidCursor,
    /// Gmail answered in a shape this module cannot read.
    #[error("gmail {0} returned an unreadable body")]
    Unreadable(&'static str),
    /// Token exchange, transport or another HTTP status.
    #[error(transparent)]
    Gmail(#[from] Error),
}

/// Reads one connected mailbox's history.
pub struct History {
    endpoints: Endpoints,
    tokens: Tokens,
    http: reqwest::Client,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProfileBody {
    history_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryBody {
    #[serde(default)]
    history: Vec<Record>,
    next_page_token: Option<String>,
    history_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    #[serde(default)]
    messages_added: Vec<Change>,
    #[serde(default)]
    messages_deleted: Vec<Change>,
}

#[derive(Deserialize)]
struct Change {
    message: MessageRef,
}

#[derive(Deserialize)]
struct MessageRef {
    id: String,
}

impl History {
    /// A reader for the mailbox that granted this refresh token. Empty fields
    /// stay unconfigured and every read returns [`Error::Credentials`].
    pub fn refresh(
        endpoints: Endpoints,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        let http = http_client();
        let credentials =
            Credentials::refresh(client_id.into(), client_secret.into(), refresh_token.into());
        Self {
            tokens: Tokens::new(
                endpoints.token.clone(),
                GMAIL_READONLY_SCOPE,
                credentials,
                http.clone(),
            ),
            endpoints,
            http,
        }
    }

    /// The cursor for a newly connected mailbox: its current history id.
    pub async fn first_cursor(&self) -> Result<Cursor, ListError> {
        let response = self.get(&self.endpoints.profile, &[]).await?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::Status {
                what: "profile",
                status: status.as_u16(),
            }
            .into());
        }
        let profile: ProfileBody = response
            .json()
            .await
            .map_err(|_| ListError::Unreadable("profile"))?;
        if !history_id_shape(&profile.history_id) {
            return Err(ListError::Unreadable("profile"));
        }
        Ok(Cursor {
            start: profile.history_id,
            page: None,
        })
    }

    /// The page at `cursor`, and where the next read starts.
    pub async fn list(&self, cursor: &Cursor) -> Result<Page, ListError> {
        let mut query = vec![("startHistoryId", cursor.start.as_str())];
        if let Some(page) = &cursor.page {
            query.push(("pageToken", page.as_str()));
        }
        let response = self.get(&self.endpoints.history, &query).await?;
        let status = response.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ListError::CursorExpired);
        }
        if !status.is_success() {
            return Err(Error::Status {
                what: "history",
                status: status.as_u16(),
            }
            .into());
        }
        let body: HistoryBody = response
            .json()
            .await
            .map_err(|_| ListError::Unreadable("history"))?;
        if !history_id_shape(&body.history_id) {
            return Err(ListError::Unreadable("history"));
        }
        let mut messages_added = Vec::new();
        let mut messages_deleted = Vec::new();
        for record in body.history {
            messages_added.extend(record.messages_added.into_iter().map(|c| c.message.id));
            messages_deleted.extend(record.messages_deleted.into_iter().map(|c| c.message.id));
        }
        let next_page = body.next_page_token.filter(|token| !token.is_empty());
        let has_more = next_page.is_some();
        let next = match next_page {
            // Mid-listing: keep the start, so an uncommitted page is reread.
            Some(page) => Cursor {
                start: cursor.start.clone(),
                page: Some(page),
            },
            // Last page: only now move to Gmail's history id.
            None => Cursor {
                start: body.history_id,
                page: None,
            },
        };
        Ok(Page {
            messages_added,
            messages_deleted,
            next,
            has_more,
        })
    }

    async fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<reqwest::Response, Error> {
        let access = self.tokens.access_token().await?;
        self.http
            .get(url)
            .bearer_auth(access)
            .query(query)
            .send()
            .await
            .map_err(|_| Error::Unreachable("gmail history did not reach the API"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    fn reader(fake: &testing::FakeGmail) -> History {
        History::refresh(fake.endpoints(), "client", "secret", "refresh")
    }

    #[test]
    fn a_stored_cursor_round_trips_and_foreign_ones_are_refused() {
        let cursor = Cursor {
            start: "42".into(),
            page: Some("p".into()),
        };
        assert_eq!(Cursor::decode(&cursor.encode()).unwrap(), cursor);
        let longest = format!(r#"{{"start":"{}","page":null}}"#, "9".repeat(40));
        assert!(Cursor::decode(&longest).is_ok());
        let too_long = format!(r#"{{"start":"{}","page":null}}"#, "9".repeat(41));
        for stored in [
            "",
            "{}",
            r#"{"start":"x","page":null}"#,
            r#"{"start":"-1","page":null}"#,
            r#"{"start":"","page":null}"#,
            r#"{"start":"1","page":""}"#,
            r#"{"start":1,"page":null}"#,
            &too_long,
        ] {
            assert!(
                matches!(Cursor::decode(stored), Err(ListError::InvalidCursor)),
                "{stored}"
            );
        }
    }

    #[tokio::test]
    async fn the_cursor_moves_to_gmails_history_id_only_on_the_last_page() {
        let fake = testing::spawn(false).await;
        let reader = reader(&fake);
        let first = reader.first_cursor().await.unwrap();
        {
            let mut mailbox = fake.mailbox.lock().unwrap();
            mailbox.page_size = 2;
            for id in ["m1", "m2", "m3"] {
                mailbox.deliver(id);
            }
        }
        let one = reader.list(&first).await.unwrap();
        assert_eq!(one.messages_added, ["m1", "m2"]);
        assert!(one.has_more);
        assert_eq!(one.next.start, first.start, "moved before the last page");
        assert!(one.next.page.is_some());

        let two = reader.list(&one.next).await.unwrap();
        assert_eq!(two.messages_added, ["m3"]);
        assert!(!two.has_more);
        assert_eq!(two.next.page, None);
        assert_eq!(
            two.next.start,
            fake.mailbox.lock().unwrap().history_id.to_string()
        );

        let idle = reader.list(&two.next).await.unwrap();
        assert!(idle.messages_added.is_empty());
        assert_eq!(idle.next, two.next);
    }

    #[tokio::test]
    async fn a_404_for_the_stored_start_is_an_expired_cursor() {
        let fake = testing::spawn(false).await;
        let reader = reader(&fake);
        let first = reader.first_cursor().await.unwrap();
        {
            let mut mailbox = fake.mailbox.lock().unwrap();
            mailbox.deliver("m1");
            mailbox.expire_through_current();
        }
        assert!(matches!(
            reader.list(&first).await,
            Err(ListError::CursorExpired)
        ));
    }

    #[tokio::test]
    async fn missing_credentials_read_nothing() {
        let fake = testing::spawn(false).await;
        let reader = History::refresh(fake.endpoints(), "", "", "");
        assert!(matches!(
            reader.first_cursor().await,
            Err(ListError::Gmail(Error::Credentials))
        ));
        assert_eq!(fake.mailbox.lock().unwrap().reads, 0);
    }
}
