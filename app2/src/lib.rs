use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "client")]
pub mod client {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    use super::App;

    #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=|| view! { <p>"app2"</p> }/>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "server")]
pub mod server {
    use axum::Router;
    use leptos::*;
    use leptos_axum::*;
    use tower_http::services::ServeDir;

    pub async fn service(options: LeptosOptions) -> Router {
        let routes = generate_route_list(super::App);

        Router::new()
            .leptos_routes(&options.clone(), routes, super::App)
            .fallback_service(ServeDir::new(&options.site_root.clone()))
            .with_state(options)
    }
}
