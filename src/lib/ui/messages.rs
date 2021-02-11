use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tui::widgets::{Block, Borders, List, ListItem};
use uuid::Uuid;

use crate::entity::proto::Parcel;

pub struct Messages {
    chat_id: Uuid,
    messages: Arc<Mutex<Vec<Parcel>>>,
    #[allow(dead_code)]
    input_handle: JoinHandle<()>,
}

impl Messages {
    pub fn new(chat_id: Uuid, mut in_message_rx: Receiver<Parcel>) -> Self {
        let messages = Arc::new(Mutex::new(Vec::new()));

        let input_handle = {
            let messages = messages.clone();

            tokio::spawn(async move {
                while let Some(message) = in_message_rx.recv().await {
                    match messages.lock() {
                        Ok(mut lock) => lock.push(message),
                        Err(e) => {
                            eprintln!("{:?}", e);
                        }
                    }
                }
            })
        };

        Self {
            chat_id,
            messages,
            input_handle,
        }
    }

    pub fn draw(&mut self) -> Result<List> {
        if let Ok(lock) = self.messages.try_lock() {
            let items = lock
                .iter()
                .map(|m| match m {
                    Parcel::LocalMessage(msg) => {
                        ListItem::new(format!("[{}]: {}", msg.author.name, msg.body))
                    }
                })
                .collect::<Vec<ListItem>>();
            let block = List::new(items).block(Block::default().borders(Borders::ALL));

            return Ok(block);
        }

        Ok(List::new(vec![]).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Chat: {}", self.chat_id)),
        ))
    }
}
