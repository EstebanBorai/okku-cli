use anyhow::Result;
use futures::SinkExt;
use futures::StreamExt;
use serde_json::from_str as json_from_str;
use std::str::FromStr;
use tokio;
use tokio::sync::mpsc::channel;
use tokio_tungstenite::tungstenite::Message;

use crate::api::{Api, WebSocket};
use crate::config::Config;
use crate::entity::proto::{OutputParcel, Parcel};

use crate::ui::UI;

pub async fn run(config: &Config) -> Result<()> {
    let okku_api = Api::new(config);
    let login_response = okku_api
        .login(config.username.as_str(), config.password.as_str())
        .await?;

    let me_response = okku_api.me(&login_response.token).await?;
    let current_user_id = me_response.user.id;

    println!(
        "Logged as {} (ID: {})",
        me_response.user.name, me_response.user.id
    );

    // This field must be gathered from the consumer of the application
    let chat_id = uuid::Uuid::from_str("aa52d536-19dc-4d03-820d-8ec1302c295b").unwrap();

    let ws = WebSocket::new(
        config.server_address.as_str(),
        login_response.token.as_str(),
    )
    .await?;

    let (mut sink, mut stream) = ws.stream.split();
    let (out_pcl_tx, mut out_pcl_rx) = channel::<OutputParcel>(1024);
    let (in_message_tx, in_message_rx) = channel::<Parcel>(1024);
    let ui = UI::new(current_user_id, chat_id, out_pcl_tx, in_message_rx);

    let read_proc = tokio::spawn(async move {
        while let Some(proto) = stream.next().await {
            if let Ok(message) = proto {
                match json_from_str(message.to_string().as_str()) {
                    Ok(msg) => match in_message_tx.send(msg).await {
                        Ok(_) => {}
                        Err(e) => eprintln!("An error ocurred! {}", e.to_string()),
                    },
                    Err(e) => eprintln!("{}", e.to_string()),
                }
            }
        }
    });

    let write_proc = tokio::spawn(async move {
        use serde_json::to_string as json_stringify;

        while let Some(out_pcl) = out_pcl_rx.recv().await {
            if let Err(e) = sink
                .send(Message::Text(json_stringify(&out_pcl).unwrap()))
                .await
            {
                eprintln!("An error ocurred sending message! {}", e.to_string());
            }
        }
    });

    let ui_proc = tokio::spawn(ui.draw().await.unwrap());

    tokio::select! {
        _ = read_proc => {},
        _ = write_proc => {},
        _ = ui_proc => {},
    }

    Ok(())
}
