mod file_serve;

use crate::file_serve::file_and_error_handler;
use app::*;
use axum::{extract::FromRef, routing::post, Router};
use leptos::LeptosOptions;
use leptos::*;
use leptos_axum::generate_route_list;
use leptos_axum::LeptosRoutes;

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    leptos_options: LeptosOptions,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
    };

    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/api", post(leptos_axum::handle_server_fns))
        .leptos_routes(&app_state, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // let app = user::add_auth(app, &app_state.pool)
    //     .with_state(app_state);

    let addr = leptos_options.site_addr;
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
