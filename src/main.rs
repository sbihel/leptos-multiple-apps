use leptos::{
    leptos_config::{Env, ReloadWSProtocol},
    *,
};

#[tokio::main]
async fn main() {
    let app1_config = LeptosOptions {
        output_name: "app1".into(),
        site_root: "target/site/app1".into(),
        site_pkg_dir: "pkg".into(),
        env: Env::PROD,
        site_addr: "0.0.0.0:3000".parse().unwrap(),
        reload_port: 3101,
        reload_external_port: None,
        reload_ws_protocol: ReloadWSProtocol::WS,
        not_found_path: "site_root/404.html".into(),
    };
    let app1_service = app1::server::service(app1_config.clone())
        .await
        .into_make_service();

    let app1_server = async {
        let addr = app1_config.site_addr;
        axum::Server::bind(&addr).serve(app1_service).await.unwrap()
    };

    let app2_config = LeptosOptions {
        output_name: "app2".into(),
        site_root: "target/site/app2".into(),
        site_pkg_dir: "pkg".into(),
        env: Env::PROD,
        site_addr: "0.0.0.0:3001".parse().unwrap(),
        reload_port: 3101,
        reload_external_port: None,
        reload_ws_protocol: ReloadWSProtocol::WS,
        not_found_path: "site_root/404.html".into(),
    };
    let app2_service = app2::server::service(app2_config.clone())
        .await
        .into_make_service();

    let app2_server = async {
        let addr = app2_config.site_addr;
        axum::Server::bind(&addr).serve(app2_service).await.unwrap()
    };

    let _ = tokio::join!(app1_server, app2_server);
}
