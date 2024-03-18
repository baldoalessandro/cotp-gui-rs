use leptonic::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let greet = move || {
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <H2>"Welcome to Leptonic"</H2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_click=move|_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>

            <TextInput get=name set=set_name placeholder="Enter a name..."/>
            <Button on_click=move |_| greet()>
                "Greet"
            </Button>

            <p><b>{ move || greet_msg.get() }</b></p>
        </Box>
    }
}
