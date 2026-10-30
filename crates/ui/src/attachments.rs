//! Files the person adds to what they write: each as a tile, its picture when it is an image,
//! otherwise its mark, name and what it is, while it uploads and once it is there, in the composer
//! with a way to remove it and in the thread with a way to open it. Choosing or dropping files is
//! the host's to read; the upload and its progress are the host's to report.

use dioxus::html::FileData;
use dioxus::prelude::*;

use crate::{Icon, IconName, Spinner};

/// Where an attachment stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentState {
    /// There, ready to send or open.
    #[default]
    Ready,
    /// On its way up.
    Uploading,
    /// Didn't upload.
    Failed,
}

/// One file the person added: its picture, from `image`, when it is an image, otherwise its mark,
/// its `name` and a quieter `detail`, such as its kind and size. Uploading, it turns, or fills its
/// foot as far as the host's `progress` (from 0 to 1) says; when it didn't upload, an image too
/// stands as a tile, its mark the alert and its outline red, its detail saying why. With `onclick`
/// the whole tile opens it; with `on_remove` a small control at its corner removes it.
#[component]
pub fn Attachment(
    name: String,
    detail: Option<String>,
    image: Option<String>,
    #[props(default)] state: AttachmentState,
    progress: Option<f64>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] remove_label: String,
    on_remove: Option<EventHandler<MouseEvent>>,
) -> Element {
    let image = image.filter(|_| state != AttachmentState::Failed);
    let mut class = String::from("attachment");
    if image.is_some() {
        class.push_str(" is-image");
    }
    class.push_str(match state {
        AttachmentState::Ready => "",
        AttachmentState::Uploading => " is-uploading",
        AttachmentState::Failed => " is-failed",
    });
    let mark = match state {
        AttachmentState::Failed => rsx! { Icon { name: IconName::Alert } },
        AttachmentState::Uploading if progress.is_none() => rsx! { Spinner {} },
        _ => rsx! { Icon { name: IconName::File } },
    };
    let filled = progress.map(|done| (done.clamp(0.0, 1.0) * 100.0).round());
    rsx! {
        div { class, aria_busy: (state == AttachmentState::Uploading).then_some("true"),
            if let Some(src) = image {
                img { class: "attachment-image", src, alt: "{name}" }
                if state == AttachmentState::Uploading && progress.is_none() {
                    span { class: "attachment-image-mark", {mark} }
                }
                if let Some(detail) = detail {
                    span { class: "sr-only", "{detail}" }
                }
            } else {
                span { class: "attachment-mark", {mark} }
                span { class: "attachment-text",
                    span { class: "attachment-name", "{name}" }
                    if let Some(detail) = detail {
                        span { class: "attachment-detail", "{detail}" }
                    }
                }
            }
            if let Some(open) = onclick {
                button { class: "attachment-open", r#type: "button", aria_label: "{name}", onclick: move |event| open.call(event) }
            }
            if let (Some(percent), AttachmentState::Uploading) = (filled, state) {
                span {
                    class: "attachment-progress",
                    role: "progressbar",
                    aria_label: "{name}",
                    aria_valuemin: "0",
                    aria_valuemax: "100",
                    aria_valuenow: "{percent}",
                    style: "--progress: {percent}%",
                }
            }
            if let Some(remove) = on_remove.filter(|_| !remove_label.is_empty()) {
                button {
                    class: "attachment-remove",
                    r#type: "button",
                    aria_label: "{remove_label}",
                    onclick: move |event| remove.call(event),
                    Icon { name: IconName::X }
                }
            }
        }
    }
}

/// Attachments together: in a `Composer`'s context, one row that scrolls across; in a `Message`,
/// wrapping at the bubble's edge.
#[component]
pub fn Attachments(children: Element) -> Element {
    rsx! { div { class: "attachments", {children} } }
}

/// A quiet icon control that opens the system's file chooser, named by `label`, taking the kinds
/// in `accept` and, when `multiple`, several at once; the files chosen go to `on_files`.
#[component]
pub fn AttachButton(
    label: String,
    accept: Option<String>,
    #[props(default)] multiple: bool,
    on_files: EventHandler<Vec<FileData>>,
) -> Element {
    rsx! {
        label { class: "control control-quiet control-icon control-compact attach-button",
            input {
                class: "sr-only",
                r#type: "file",
                aria_label: "{label}",
                accept,
                multiple,
                onchange: move |event| on_files.call(event.files()),
            }
            Icon { name: IconName::Plus }
        }
    }
}
