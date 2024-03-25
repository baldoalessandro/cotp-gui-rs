use leptonic::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct UnlockDBArgs<'a> {
    password: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let unlock_disabled = create_memo(move |_| password.with(String::is_empty));
    let (wrong_password, set_wrong_password) = create_signal(false);

    let unlock_db = move |evt: MouseEvent| {
        evt.prevent_default();
        spawn_local(async move {
            let password = password.get_untracked();
            if password.is_empty() {
                return;
            }

            let args = to_value(&UnlockDBArgs {
                password: &password,
            })
            .unwrap();
            match invoke("unlock_db", args).await.as_bool().unwrap() {
                true => {
                    let navigate = leptos_router::use_navigate();
                    navigate("/db", Default::default());
                }
                false => set_wrong_password.set(true),
            }
        });
    };

    let clear_input = move |focused: bool| {
        if !password.get_untracked().is_empty() && focused && wrong_password.get_untracked() {
            set_password.set("".to_string());
            set_wrong_password.set(false);
        }
    };

    view! {
        <Box class="screen">
            <Stack spacing=Size::Em(0.6)>

                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                    <Icon icon=icondata::CgPassword style="font-size: 3em;"/>
                    <h1>COTP</h1>
                </Stack>

                <Stack spacing=Size::Em(0.3)>
                    <Field>
                        {move || match wrong_password.get() {
                            false => {
                                view! {
                                    <FieldLabel style="margin-bottom: 0.6em; color: var(--secondary-color)">
                                        "Insert master password ..."
                                    </FieldLabel>
                                }
                            }
                            true => {
                                view! {
                                    <FieldLabel style="margin-bottom: 0.6em; color: var(--danger-color)">
                                        "Wrong password!"
                                    </FieldLabel>
                                }
                            }
                        }}
                        <PasswordInput get=password set=set_password on_focus_change=clear_input/>
                    </Field>
                    <Button
                        on_click=unlock_db
                        size=ButtonSize::Big
                        style="width: 100%;"
                        disabled=unlock_disabled
                    >
                        "Unlock DB"
                    </Button>
                </Stack>

            </Stack>
        </Box>
    }
}
