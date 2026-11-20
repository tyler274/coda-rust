//! Ephemeral account export presentation; bytes come from freshly verified account exports.

/// Present full account export JSON via browser download when on web.
pub fn present_account_export(bytes: &[u8]) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().ok_or("Browser unavailable")?;
        let array = js_sys::Uint8Array::from(bytes);
        let parts = js_sys::Array::new();
        parts.push(&array);
        let options = web_sys::BlobPropertyBag::new();
        options.set_type("application/json");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)
            .map_err(|_| "Cannot prepare download")?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|_| "Cannot prepare download")?;
        let result = (|| {
            let document = window.document().ok_or("Document unavailable")?;
            let link = document
                .create_element("a")
                .map_err(|_| "Download unavailable")?
                .dyn_into::<web_sys::HtmlAnchorElement>()
                .map_err(|_| "Download unavailable")?;
            link.set_href(&url);
            link.set_download("duet-account-export.json");
            document
                .body()
                .ok_or("Document unavailable")?
                .append_child(&link)
                .map_err(|_| "Download unavailable")?;
            link.click();
            link.remove();
            Ok(())
        })();
        // A short grace period allows browser download acquisition; no PHI goes to browser storage.
        let revoke = url.clone();
        dioxus::prelude::spawn(async move {
            futures_timer::Delay::new(std::time::Duration::from_secs(1)).await;
            let _ = web_sys::Url::revoke_object_url(&revoke);
        });
        if result.is_err() {
            let _ = web_sys::Url::revoke_object_url(&url);
        }
        result
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = bytes;
        Ok(())
    }
}
