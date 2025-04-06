#![deny(unsafe_code)]

pub fn main() -> anyhow::Result<()> {
    #[cfg(any(
        not(any(feature = "web", feature = "server")),
        all(feature = "web", feature = "server")
    ))]
    {
        compile_error!("exactly one of either feature must be enabled: web, server");
    }

    #[cfg(feature = "web")]
    {
        use dioxus::LaunchBuilder;

        tsto_server::logger::init()?;
        LaunchBuilder::web().launch(tsto_server::app::dashboard::App);

        Ok(())
    }

    #[cfg(feature = "server")]
    {
        use tracing::info;
        use tsto_server::util::UPTIME;

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tsto_server::logger::init()?;
                tsto_server::database::init().await?;

                info!("initializing server");
                let router = tsto_server::app::create_router().await?;
                let listener = tsto_server::app::create_listener().await?;

                // access UPTIME to initialize it as it is behind a lazy_static
                UPTIME.elapsed().ok();

                info!("listening on {}", listener.local_addr().unwrap());
                axum::serve(listener, router).await?;

                Ok(())
            })
    }
}
