use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tui::widgets::{Block, Borders, List, ListItem};
use uuid::Uuid;

use crate::entity::proto::Parcel;

pub struct Messages {
    messages: Arc<Mutex<Vec<Parcel>>>,
    input_handle: JoinHandle<()>,
}

impl Messages {
    pub fn new(chat_id: Uuid, mut in_pcl_rx: Receiver<Parcel>) -> Self {
        let messages = Arc::new(Mutex::new(Vec::new()));

        let input_handle = {
            let messages = messages.clone();

            tokio::spawn(async move {
                for message in in_pcl_rx.recv().await {
                    messages.clone().lock().unwrap().push(message);
                }
            })
        };

        Self {
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

        Ok(List::new(vec![]).block(Block::default().borders(Borders::ALL)))
    }
}
