//! Physician journeys: Home, an open workspace, chat, and the reader.
//! The React app remains the shipping client until these match it.

use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{PatientWorkspaceResponse, PatientWorkspacesResponse, TodayListResponse, WorkspaceChat};
use ui::{Card, EmptyState, ErrorState, PageTitle, Row, TabBar, TextButton};

const TABS: [&str; 4] = ["Home", "Workspace", "Chat", "Reader"];

#[component]
pub fn PhysicianRoot(base_url: String) -> Element {
    let mut tab = use_signal(|| 0usize);
    let mut selected = use_signal(|| None::<String>);
    let client = use_signal(|| PhysicianClient::new(base_url));
    let today = use_resource(move || {
        let client = client.read().clone();
        async move { client.today().await.map_err(|error| error.to_string()) }
    });
    let workspaces = use_resource(move || {
        let client = client.read().clone();
        async move { client.workspaces().await.map_err(|error| error.to_string()) }
    });
    let open_id = selected.read().clone();
    let workspace = use_resource(move || {
        let client = client.read().clone();
        let open_id = open_id.clone();
        async move {
            match open_id {
                Some(id) => client.workspace(&id).await.map_err(|error| error.to_string()).map(Some),
                None => Ok(None),
            }
        }
    });
    rsx! {
        style { "{ui::STYLESHEET}" }
        main { class: "app",
            div { class: "page",
                match tab() {
                    0 => rsx! { Home { today: today(), on_open: move |id| { selected.set(Some(id)); tab.set(1); } } },
                    1 => rsx! { Workspace { workspaces: workspaces(), open: workspace(), on_open: move |id| selected.set(Some(id)) } },
                    2 => rsx! { Chat { open: workspace() } },
                    _ => rsx! { Reader { today: today() } },
                }
            }
            TabBar {
                labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                selected: tab(),
                on_select: move |index| tab.set(index),
            }
        }
    }
}

#[component]
fn Home(today: Option<Result<TodayListResponse, String>>, on_open: EventHandler<String>) -> Element {
    rsx! {
        PageTitle { text: "Today".to_owned() }
        Card {
            match today {
                None => rsx! { EmptyState { message: "Loading Today.".to_owned() } },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                Some(Ok(list)) if list.rows.is_empty() => rsx! { EmptyState { message: "Nothing is waiting.".to_owned() } },
                Some(Ok(list)) => rsx! {
                    for row in list.rows {
                        div {
                            Row { name: row.patient_display_name.clone(), detail: row.reason.clone().unwrap_or(row.kind.clone()) }
                            TextButton {
                                label: "Open".to_owned(),
                                quiet: true,
                                on_press: {
                                    let id = row.patient_workspace_id.clone();
                                    move |_| on_open.call(id.clone())
                                },
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn Workspace(workspaces: Option<Result<PatientWorkspacesResponse, String>>, open: Option<Result<Option<PatientWorkspaceResponse>, String>>, on_open: EventHandler<String>) -> Element {
    rsx! {
        PageTitle { text: "Workspace".to_owned() }
        Card {
            match workspaces {
                None => rsx! { EmptyState { message: "Loading patients.".to_owned() } },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                Some(Ok(list)) if list.patients.is_empty() => rsx! { EmptyState { message: "No patients are on this list.".to_owned() } },
                Some(Ok(list)) => rsx! {
                    for patient in list.patients {
                        TextButton {
                            label: patient.display_name.clone(),
                            quiet: true,
                            on_press: {
                                let id = patient.patient_workspace_id.clone();
                                move |_| on_open.call(id.clone())
                            },
                        }
                    }
                },
            }
            match open {
                Some(Ok(Some(workspace))) => rsx! { Row { name: workspace.display_name, detail: "Workspace open.".to_owned() } },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                _ => rsx! {},
            }
        }
    }
}

#[component]
fn Chat(open: Option<Result<Option<PatientWorkspaceResponse>, String>>) -> Element {
    rsx! {
        PageTitle { text: "Chat".to_owned() }
        Card {
            match open {
                None => rsx! { EmptyState { message: "Loading chats.".to_owned() } },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                Some(Ok(None)) => rsx! { EmptyState { message: "Open a workspace to see its chats.".to_owned() } },
                Some(Ok(Some(workspace))) if workspace.chats.is_empty() => rsx! { EmptyState { message: "This workspace has no chats.".to_owned() } },
                Some(Ok(Some(workspace))) => rsx! {
                    for chat in workspace.chats {
                        ChatRow { chat }
                    }
                },
            }
        }
    }
}

#[component]
fn ChatRow(chat: WorkspaceChat) -> Element {
    let title = chat.title.unwrap_or_else(|| "Untitled".to_owned());
    rsx! { Row { name: title, detail: chat.created_at } }
}

#[component]
fn Reader(today: Option<Result<TodayListResponse, String>>) -> Element {
    rsx! {
        PageTitle { text: "Reader".to_owned() }
        Card {
            match today {
                Some(Ok(list)) => rsx! {
                    if let Some(row) = list.rows.first() {
                        Row { name: row.patient_display_name.clone(), detail: "Open the source from this row. The citation text stays on the server read.".to_owned() }
                    } else {
                        EmptyState { message: "Nothing is open to read.".to_owned() }
                    }
                },
                Some(Err(error)) => rsx! { ErrorState { message: error } },
                None => rsx! { EmptyState { message: "Loading the reader.".to_owned() } },
            }
        }
    }
}
