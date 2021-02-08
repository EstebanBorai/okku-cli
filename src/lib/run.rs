use std::str::FromStr;

use anyhow::Result;
use futures::{future::ready, SinkExt};
use futures::{StreamExt, TryStreamExt};
use tokio;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::Message;

use crate::api::{Api, WebSocket};
use crate::config::Config;
use crate::entity::proto::{OutputParcel, OutputParcelPayload};
use crate::io::readstdin;

use crate::ui::UI;

pub async fn run(config: &Config) -> Result<()> {
    // let okku_api = Api::new(config);
    // let login_response = okku_api
    //     .login(config.username.as_str(), config.password.as_str())
    //     .await?;

    // let me_response = okku_api.me(&login_response.token).await?;
    // let current_user_id = me_response.user.id;

    // println!(
    //     "Logged as {} (ID: {})",
    //     me_response.user.name, me_response.user.id
    // );

    // // This field must be gathered from the consumer of the application
    // let chat_id = uuid::Uuid::from_str("78b26b86-cee6-455b-9e7b-d07394a54602").unwrap();

    // let ws = WebSocket::new(
    //     config.server_address.as_str(),
    //     login_response.token.as_str(),
    // )
    // .await?;

    // let (mut sink, stream) = ws.stream.split();
    // let (stdin_tx, mut stdin_rx) = unbounded_channel::<Vec<u8>>();
    // let (out_pcl_tx, mut out_pcl_rx) = channel::<OutputParcel>(1024);

    // let readstdin_proc = tokio::spawn(readstdin(stdin_tx));
    // let handle_stdin_rx_proc = tokio::spawn(async move {
    //     use chrono::Utc;
    //     use std::str::from_utf8;

    //     while let Some(stdin_bytes) = stdin_rx.recv().await {
    //         let utf8 = from_utf8(stdin_bytes.as_slice()).unwrap();

    //         if let Err(e) = out_pcl_tx
    //             .send(OutputParcel {
    //                 inner: OutputParcelPayload {
    //                     author_id: current_user_id,
    //                     chat_id: chat_id,
    //                     body: utf8.to_string(),
    //                     created_at: Utc::now(),
    //                 },
    //             })
    //             .await
    //         {
    //             println!("An error ocurred parsing message! {}", e.to_string());
    //         }
    //     }
    // });

    // let read_proc = tokio::spawn(async move {
    //     stream
    //         .try_for_each(|proto| {
    //             // receiving message as text
    //             // must turn into struct via serde json
    //             println!("{:?}", proto);
    //             ready(Ok(()))
    //         })
    //         .await
    //         .unwrap();
    // });

    // let write_proc = tokio::spawn(async move {
    //     use serde_json::to_string as json_stringify;

    //     while let Some(out_pcl) = out_pcl_rx.recv().await {
    //         if let Err(e) = sink
    //             .send(Message::Text(json_stringify(&out_pcl).unwrap()))
    //             .await
    //         {
    //             println!("An error ocurred sending message! {}", e.to_string());
    //         }
    //     }
    // });

    // tokio::select! {
    //     _ = readstdin_proc => {},
    //     _ = handle_stdin_rx_proc => {},
    //     _ = read_proc => {},
    //     _ = write_proc => {},
    // }
    UI::draw();

    Ok(())
}
