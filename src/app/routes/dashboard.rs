use crate::{
    config::OPTIONS,
    util::{relative_path, DIRECTORIES},
};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures::Stream;
use std::{
    io::{BufRead, BufReader},
    time::Duration,
};
use tracing::error;

// /dashboard
pub fn create_router() -> Router {
    Router::new().route("/logs", get(get_logs))
}
#[tracing::instrument]
async fn get_logs(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    use axum::extract::ws::Message;
    use futures::StreamExt;

    match get_logs_stream() {
        Ok(stream) => {
            let mut pinned_stream = Box::pin(stream);
            'outer: loop {
                while let Some(log) = pinned_stream.next().await {
                    if let Err(_e) = socket.send(Message::Text(log)).await {
                        #[cfg(debug_assertions)]
                        tracing::warn!("failed to send ws message: {_e}");
                        break 'outer;
                    }
                }
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
            if let Err(_e) = socket.close().await {
                #[cfg(debug_assertions)]
                tracing::warn!("failed to close socket: {_e}");
            }
        }
        Err(e) => error!("failed to open log stream: {e}"),
    }
}

fn get_logs_stream() -> Result<impl Stream<Item = String>, std::io::Error> {
    let mut log_file = if OPTIONS.take().portable {
        relative_path()?
    } else {
        DIRECTORIES.data_local_dir().to_path_buf()
    };
    log_file.push("server_log.jsonl");

    let logs = std::fs::OpenOptions::new().read(true).open(log_file)?;
    let reader = BufReader::new(logs);

    Ok(futures::stream::iter(
        reader.lines().map(|line| line.unwrap_or_default()),
    ))
}
