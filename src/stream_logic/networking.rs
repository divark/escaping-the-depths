use crate::core_logic::interacting::ViewerClick;
use bevy::prelude::*;
use serde_json::Value;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::{runtime::Runtime, task::JoinHandle};
use tungstenite::connect;

#[derive(Resource)]
pub struct TwitchClickListener {
    _rt: Option<Runtime>,
    _heat_click_listener: JoinHandle<()>,
    message_receiver: UnboundedReceiver<ViewerClick>,
}

fn get_viewer_click_from_response(response_text: &str) -> Option<ViewerClick> {
    let json_response = serde_json::from_str::<Value>(response_text).ok()?;
    if json_response.get("type")?.as_str()? != "click" {
        return None;
    }

    let uv_x = json_response.get("x")?.as_str()?.parse::<f32>().ok()?;
    let uv_y = json_response.get("y")?.as_str()?.parse::<f32>().ok()?;

    Some(ViewerClick::new(uv_x, 1.0 - uv_y))
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

                    if let Some(viewer_click) = get_viewer_click_from_response(response_text) {
                        let write_status = message_writer.send(viewer_click);

                        if write_status.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Self {
            _rt: Some(_rt),
            _heat_click_listener,
            message_receiver,
        }
    }

    pub fn read(&mut self) -> Option<ViewerClick> {
        self.message_receiver.try_recv().ok()
    }
}

impl Drop for TwitchClickListener {
    fn drop(&mut self) {
        let threads_running = self._rt.take().unwrap();

        // Without this, the game will listen to the
        // heat endpoint until it has been disconnected.
        //
        // If the game shuts down, we want it to disconnect
        // right away. This makes it happen by killing the threads
        // immediately.
        threads_running.shutdown_background();
    }
}

pub fn map_twitch_clicks_to_uv(
    mut twitch_click_listener: ResMut<TwitchClickListener>,
    mut viewer_click_broadcaster: EventWriter<ViewerClick>,
) {
    while let Some(twitch_click) = twitch_click_listener.read() {
        println!(
            "{}: Received click event. x = {}, y = {}",
            chrono::Local::now(),
            twitch_click.get_x(),
            twitch_click.get_y()
        );
        viewer_click_broadcaster.write(twitch_click);
    }
}
