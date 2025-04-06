#![deny(unsafe_code)]

#[cfg(any(
    not(any(feature = "web", feature = "server")),
    all(feature = "web", feature = "server")
))]
fn main() {
    compile_error!("exactly one of either feature must be enabled: web, server");
}

#[cfg(feature = "web")]
fn main() -> anyhow::Result<()> {
    use dioxus::LaunchBuilder;

    tsto_server::logger::init()?;
    LaunchBuilder::web().launch(tsto_server::app::dashboard::App);

    Ok(())
}

#[cfg(feature = "server")]
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    use tracing::info;

    tsto_server::logger::init()?;
    tsto_server::database::init().await?;

    info!("initializing server");
    let router = tsto_server::app::create_router().await?;
    let listener = tsto_server::app::create_listener().await?;

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await?;

    Ok(())
}
