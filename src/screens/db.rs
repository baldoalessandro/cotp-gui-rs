use leptonic::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::JsValue;
use serde_wasm_bindgen::from_value;

use crate::tauri::invoke;


async fn load_otps() -> Vec<String> {
    let res = invoke("list_otps", JsValue::undefined()).await;
    from_value(res).unwrap()
}

#[component]
pub fn DB() -> impl IntoView {
    let otps = create_resource(|| (), |_| async move { load_otps().await });

    view! {
        <Box class="screen">
            <Stack spacing=Size::Em(0.6)>
                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>{move || { otps.get().map(|otp| view! { <p>{otp}</p> }) }}</Suspense>
            </Stack>
        </Box>
    }
}
