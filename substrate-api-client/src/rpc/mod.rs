use std::sync::mpsc::Sender as ThreadOut;
use std::thread;

use ws::{connect, Result as WsResult};

pub use client::XtStatus;
use client::*;

mod client;
pub mod json_req;

pub fn get(url: String, json_req: String, result_in: ThreadOut<String>) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_get_request_msg)
}

pub fn send_extrinsic(url: String, json_req: String, result_in: ThreadOut<String>) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_ready)
}

pub fn send_extrinsic_and_wait_until_broadcast(
    url: String,
    json_req: String,
    result_in: ThreadOut<String>,
) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_broadcast)
}

pub fn send_extrinsic_and_wait_until_in_block(
    url: String,
    json_req: String,
    result_in: ThreadOut<String>,
) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_in_block)
}

pub fn send_extrinsic_and_wait_until_finalized(
    url: String,
    json_req: String,
    result_in: ThreadOut<String>,
) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_finalized)
}

pub fn start_subscriber(
    url: String,
    json_req: String,
    result_in: ThreadOut<String>,
) -> WsResult<()> {
    start_rpc_client_thread(url, json_req, result_in, on_subscription_msg)
}

fn start_rpc_client_thread(
    url: String,
    jsonreq: String,
    result_in: ThreadOut<String>,
    on_message_fn: OnMessageFn,
) -> WsResult<()> {
    let _client =
        thread::Builder::new()
            .name("client".to_owned())
            .spawn(move || -> WsResult<()> {
                connect(url, |out| RpcClient {
                    out,
                    request: jsonreq.clone(),
                    result: result_in.clone(),
                    on_message_fn,
                })
            })?;
    Ok(())
}

type RpcResult<T> = Result<T, RpcClientError>;

#[derive(Debug, thiserror::Error)]
#[cfg(feature = "std")]
pub enum RpcClientError {
    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::error::Error),
    #[error("Extrinsic Error: {0}")]
    Extrinsic(String),
    #[error("mpsc send Error: {0}")]
    Send(#[from] std::sync::mpsc::SendError<String>),
}
