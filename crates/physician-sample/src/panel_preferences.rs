//! Server-persisted per-physician panel collapse preferences
//! (`GET /physician-state/panels`, `PUT /physician-state/panels`).
//!
//! Uses Postgres via [`Store::panel_preferences`] / [`Store::save_panel_preferences`]
//! when attached, and falls back to an in-memory map on fixture-only routers.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::http::{Extensions, StatusCode};
use axum::response::{IntoResponse, Response};
use local_product::runtime::Store;
use local_product::runtime::panel_preferences::PanelPreferences;
use physician_wire::PanelPreferencesWire;

use crate::care::ConnectedCare;

/// Process-memory fallback for panel preferences on fixture-only routers.
#[derive(Clone, Default)]
pub(crate) struct FallbackPanelPreferences(
    pub(crate) Arc<Mutex<BTreeMap<(String, String), PanelPreferencesWire>>>,
);

impl FallbackPanelPreferences {
    /// Creates an empty in-memory panel preference store.
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

/// Serves `GET /physician-state/panels` for the signed-in physician.
pub(crate) async fn read(extensions: Extensions) -> Response {
    let identity = ConnectedCare::identity(&extensions);
    if let Some(store) = extensions.get::<Store>().cloned() {
        return match store
            .panel_preferences(&identity.practice_id, &identity.physician_id)
            .await
        {
            Ok(prefs) => Json(PanelPreferencesWire {
                sidebar_collapsed: prefs.sidebar_collapsed,
                schedule_collapsed: prefs.schedule_collapsed,
            })
            .into_response(),
            Err(error) => {
                tracing::error!(%error, "panel preferences read failed");
                StatusCode::SERVICE_UNAVAILABLE.into_response()
            }
        };
    }
    let prefs = extensions
        .get::<FallbackPanelPreferences>()
        .and_then(|fallback| {
            fallback.0.lock().ok().and_then(|map| {
                map.get(&(identity.practice_id.clone(), identity.physician_id.clone()))
                    .copied()
            })
        })
        .unwrap_or_default();
    Json(prefs).into_response()
}

/// Serves `PUT /physician-state/panels` for the signed-in physician.
pub(crate) async fn save(
    extensions: Extensions,
    Json(body): Json<PanelPreferencesWire>,
) -> Response {
    let identity = ConnectedCare::identity(&extensions);
    if let Some(store) = extensions.get::<Store>().cloned() {
        return match store
            .save_panel_preferences(
                &identity.practice_id,
                &identity.physician_id,
                PanelPreferences {
                    sidebar_collapsed: body.sidebar_collapsed,
                    schedule_collapsed: body.schedule_collapsed,
                },
            )
            .await
        {
            Ok(saved) => Json(PanelPreferencesWire {
                sidebar_collapsed: saved.sidebar_collapsed,
                schedule_collapsed: saved.schedule_collapsed,
            })
            .into_response(),
            Err(error) => {
                tracing::error!(%error, "panel preferences save failed");
                StatusCode::SERVICE_UNAVAILABLE.into_response()
            }
        };
    }
    if let Some(fallback) = extensions.get::<FallbackPanelPreferences>()
        && let Ok(mut map) = fallback.0.lock()
    {
        map.insert(
            (identity.practice_id.clone(), identity.physician_id.clone()),
            body,
        );
    }
    Json(body).into_response()
}
