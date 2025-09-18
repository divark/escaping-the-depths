use std::time::Duration;

use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::core_logic::interacting::ViewerClick;
use bevy::prelude::*;
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

#[derive(Resource)]
pub struct TwitchClickListener {
    _rt: Option<Runtime>,
    message_receiver: UnboundedReceiver<ViewerClick>,
}

async fn read_heat_response(
    connection: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Option<Value> {
    let response_timeout = Duration::from_secs(30);

    let response = timeout(response_timeout, connection.next())
        .await
        .ok()??
        .ok()?;
    let response_text = response
        .into_text()
        .expect("read_heat_response: Could not convert response to text.");
    let response_text = response_text.as_str();
    let response_json = serde_json::from_str::<Value>(response_text)
        .expect("read_heat_response: Response is not in JSON.");
    Some(response_json)
}

async fn verify_connected_to_heat_api(
    connection: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    let response_json = read_heat_response(connection)
        .await
        .expect("verify_connected_to_heat_api: Could not read system response.");
    let valid_response = response_json["type"] == "system"
        && response_json["message"] == "Connected to Heat API server.";
    if !valid_response {
        panic!("verify_connected_to_heat_api: Did not get system response.");
    }
}

async fn read_click_event(
    connection: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Option<ViewerClick> {
    let json_response = read_heat_response(connection).await?;
    get_viewer_click_from_response(json_response)
}

fn get_viewer_click_from_response(json_response: Value) -> Option<ViewerClick> {
    if json_response.get("type")?.as_str()? != "click" {
        return None;
    }

    let uv_x = json_response.get("x")?.as_str()?.parse::<f32>().ok()?;
    let uv_y = json_response.get("y")?.as_str()?.parse::<f32>().ok()?;

    Some(ViewerClick::new(uv_x, 1.0 - uv_y))
}

async fn connect_to_heat_api(
    twitch_channel_id: String,
) -> SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let connection_url = format!("wss://heat-api.j38.net/channel/{}", twitch_channel_id);
    let (connection, _) = connect_async(&connection_url)
        .await
        .expect("connect: Cannot connect to heat URL.");
    let (_, mut connection_reader) = connection.split();
    verify_connected_to_heat_api(&mut connection_reader).await;

    println!("connect_to_heat_api: Successfully connected to heat endpoint.");

    connection_reader
}

impl TwitchClickListener {
    pub fn connect(channel_id: &str) -> Self {
        let _rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .enable_time()
            .build()
            .expect("connect: Could not prepare networking for click listening.");

        let twitch_channel_id = String::from(channel_id);
        let (message_writer, message_receiver) = mpsc::unbounded_channel();
        _rt.spawn(async move {
            loop {
                let mut connection_reader = connect_to_heat_api(twitch_channel_id.clone()).await;
                while let Some(click_event) = read_click_event(&mut connection_reader).await {
                    message_writer
                        .send(click_event)
                        .expect("Cannot broadcast viewer clicks anymore");
                }
            }
        });

        Self {
            _rt: Some(_rt),
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
