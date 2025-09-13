use crate::core_logic::interacting::ViewerClick;
use bevy::prelude::*;
use serde_json::Value;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::{runtime::Runtime, task::JoinHandle};
use tungstenite::connect;

#[derive(Resource)]
pub struct TwitchClickListener {
    _rt: Runtime,
    _heat_click_listener: JoinHandle<()>,
    message_receiver: UnboundedReceiver<ViewerClick>,
}

impl TwitchClickListener {
    pub fn connect(channel_id: &str) -> Self {
        let _rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("connect: Could not prepare networking for click listening.");

        let twitch_channel_id = String::from(channel_id);
        let (message_writer, message_receiver) = mpsc::unbounded_channel();
        let _heat_click_listener = _rt.spawn(async move {
            loop {
                let connection_url =
                    format!("wss://heat-api.j38.net/channel/{}", twitch_channel_id);
                let (mut connection, initial_response) =
                    connect(&connection_url).expect("connect: Cannot connect to heat URL.");
                println!(
                    "Connected to {}.\nInitial Response: {:?}",
                    connection_url, initial_response
                );

                while let Ok(response) = connection.read() {
                    if !response.is_text() {
                        continue;
                    }

                    let response_text = response.into_text().unwrap();
                    let response_text = response_text.as_str();
                    if let Ok(json_response) = serde_json::from_str::<Value>(response_text) {
                        if let Some(uv_x) = json_response["x"].as_f64() {
                            if let Some(uv_y) = json_response["y"].as_f64() {
                                let write_status =
                                    message_writer.send(ViewerClick::new(uv_x as f32, uv_y as f32));

                                if write_status.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        });

        Self {
            _rt,
            _heat_click_listener,
            message_receiver,
        }
    }

    pub fn read(&mut self) -> Option<ViewerClick> {
        if let Ok(viewer_click) = self.message_receiver.try_recv() {
            Some(viewer_click)
        } else {
            None
        }
    }
}

pub fn map_twitch_clicks_to_uv(
    mut twitch_click_listener: ResMut<TwitchClickListener>,
    mut viewer_click_broadcaster: EventWriter<ViewerClick>,
) {
    while let Some(twitch_click) = twitch_click_listener.read() {
        viewer_click_broadcaster.write(twitch_click);
    }
}
