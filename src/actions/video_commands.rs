use crate::client::discord_client;

use openaction::{Action, ActionUuid, Instance, OpenActionResult, async_trait};
use serde_json::json;
use std::collections::HashMap;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

pub struct ToggleScreenshareAction;

pub fn get_focused_pid() -> Option<u32> {
	unsafe {
		let hwnd = GetForegroundWindow();

		if hwnd.0 == std::ptr::null_mut() {
			return None;
		}

		let mut pid: u32 = 0;

		GetWindowThreadProcessId(hwnd, Some(&mut pid));

		Some(pid)
	}
}

#[async_trait]
impl Action for ToggleScreenshareAction {
	const UUID: ActionUuid = "me.radsi.oadiscord.togglescreenshare";
	type Settings = HashMap<String, String>;

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		let stream_focused_window = settings
			.get("streamFocusedWindow")
			.map(|v| v == "true")
			.unwrap_or(false);

		let focus_pid = if stream_focused_window {
			get_focused_pid()
		} else {
			None
		};

		let mut client_lock = discord_client().write().await;
		let Some(client) = client_lock.as_mut() else {
			log::error!("Discord client not initialized");
			instance.show_alert().await?;
			return Ok(());
		};

		let nonce = match focus_pid {
			Some(pid) => format!("toggle-screenshare-{pid}"),
			None => String::from("toggle-screenshare"),
		};

		let payload = json!({
			"cmd": "TOGGLE_SCREENSHARE",
			"args": {
				"pid": focus_pid,
			},
			"nonce": nonce,
		});

		let payload_text = match serde_json::to_string(&payload) {
			Ok(text) => text,
			Err(e) => {
				log::error!("Failed to serialize payload: {}", e);
				instance.show_alert().await?;
				return Ok(());
			}
		};

		if let Err(e) = client.emit_string(&payload_text).await {
			log::error!("Failed to toggle screenshare: {}", e);
			instance.show_alert().await?;
		}

		Ok(())
	}
}

pub struct ToggleCameraAction;

#[async_trait]
impl Action for ToggleCameraAction {
	const UUID: ActionUuid = "me.radsi.oadiscord.togglecamera";
	type Settings = HashMap<String, String>;

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		let mut client_lock = discord_client().write().await;
		let Some(client) = client_lock.as_mut() else {
			log::error!("Discord client not initialized");
			instance.show_alert().await?;
			return Ok(());
		};

		let payload = json!({
			"cmd": "TOGGLE_VIDEO",
			"nonce": format!("toggle-video"),
		});

		let payload_text = match serde_json::to_string(&payload) {
			Ok(text) => text,
			Err(e) => {
				log::error!("Failed to serialize payload: {}", e);
				instance.show_alert().await?;
				return Ok(());
			}
		};

		if let Err(e) = client.emit_string(&payload_text).await {
			log::error!("Failed to toggle video: {}", e);
			instance.show_alert().await?;
		}

		Ok(())
	}
}
