use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    screens::db::DB,
    screens::home::Home,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Root default_theme=LeptonicTheme::default()>

            <Meta name="charset" content="UTF-8"/>
            <Meta name="description" content="Trustworthy, encrypted authenticator app"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <Meta name="theme-color" content="#e66956"/>

            <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

            <Title text="COTP"/>

            <Box style="position: relative; width: 100%; height: 100%; overflow: none;">
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }
                }>
                    <Routes>
                        <Route path="" view=|| view! { <Home/> }/>
                        <Route path="/db" view=DB/>
                    </Routes>
                </Router>
            </Box>
        </Root>
    }
}
