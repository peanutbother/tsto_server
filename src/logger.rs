pub fn init() -> anyhow::Result<()> {
    #[cfg(feature = "web")]
    dioxus::logger::initialize_default();

    #[cfg(feature = "server")]
    {
        init_server()?;
    }

    Ok(())
}

#[cfg(feature = "server")]
fn init_server() -> anyhow::Result<()> {
    use crate::{
        config::OPTIONS,
        util::{relative_path, DIRECTORIES},
    };
    use std::fs::{create_dir_all, OpenOptions};
    use tracing::{info, Level};
    use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

    let is_debug = cfg!(debug_assertions);
    let log_assets = OPTIONS.take().log_assets;
    let is_portable = OPTIONS.take().portable;
    let crate_name = env!("CARGO_CRATE_NAME");
    let log_path = if is_portable {
        relative_path()?
    } else {
        DIRECTORIES.data_local_dir().to_path_buf()
    }
    .join("server_log.jsonl");

    {
        let parent = log_path.parent().expect("path is valid ut-8");
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    let filter = filter::filter_fn(move |metadata| {
        let target = metadata.target();
        let level = *metadata.level();
        let asset_filter = format!("{crate_name}{}", "::on_request_asset");
        let tracking_filter = format!("{crate_name}{}", "::on_tracking");
        let metrics_filter = format!("{crate_name}{}", "::on_metrics");
        let event_filter = format!("{crate_name}{}", "::on_event");
        let telemetry_filter = format!("{crate_name}{}", "::on_telemetry");
        let stats_filter = format!("{crate_name}{}", "::on_userstats");

        target.starts_with(crate_name)
            && level <= if is_debug { Level::TRACE } else { Level::INFO }
            && (log_assets || !target.starts_with(&asset_filter))
            && (is_debug
                || !target.starts_with(&tracking_filter)
                    && !target.starts_with(&metrics_filter)
                    && !target.starts_with(&event_filter)
                    && !target.starts_with(&telemetry_filter)
                    && !target.starts_with(&stats_filter))
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "none,{}={},tower_http={1},axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"),
                    if cfg!(debug_assertions) {
                        "trace"
                    } else {
                        "info"
                    }
                )
                .into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(is_debug)
                .with_file(is_debug),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_writer(
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&log_path)
                        .expect("create(open,append) log file"),
                ),
        )
        .init();

    info!(
        "writing server log to {}",
        log_path.to_str().expect("log path is valid UTF-8")
    );

    Ok(())
}

#[cfg(feature = "server")]
macro_rules! with_tracing {
    () => {
        with_tracing!(concat!(env!("CARGO_PKG_NAME"),"::on_request"), concat!(env!("CARGO_PKG_NAME"),"::on_response"))
    };
    ($target_request:expr, $target_response:expr) => {
        {
            use axum::{extract::MatchedPath, http::Request, response::Response};
            use std::time::Duration;
            use tower_http::{
                classify::ServerErrorsFailureClass,
                trace::{DefaultOnBodyChunk, TraceLayer},
            };
            use tracing::{error, info, info_span, Span};

            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let content_type = request
                    .headers()
                    .get("content-type")
                    .map(|c| c.to_str().unwrap_or_default())
                    .unwrap_or_default();

                    let accept = request
                    .headers()
                    .get("accept")
                    .map(|c| c.to_str().unwrap_or_default())
                    .unwrap_or_default();

                let span = info_span!(
                    "http_request",
                    method = ?request.method(),
                    uri = ?request.uri(),
                    matched_path,
                    content_type,
                    accept,
                );

                span
            })
            .on_request(|req: &Request<_>, _span: &Span| {
                info!(target: $target_request, "[{}] {}", req.method(), req.uri());
            })
            .on_response(|res: &Response, _latency: Duration, _span: &Span| {
                info!(target: $target_response, "{}", res.status());
            })
            .on_body_chunk(DefaultOnBodyChunk::new())
            .on_failure(
                |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    error!("{:?}", error);
                },
            )
        }
    };
}
#[cfg(feature = "server")]
pub(crate) use with_tracing;
