#![allow(non_snake_case)]
pub mod error_template;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/kleio.css" />

        <Router>
            <div class="">"Hello! Great, text node changes work "
                <Routes>
                    <Route path="" view=|_cx| view! { cx, "🌍" }/>
                </Routes>
            </div>
        </Router>
    }
}
