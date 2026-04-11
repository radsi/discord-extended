use crate::client::discord_client;

use std::collections::HashMap;

use openaction::{Action, ActionUuid, Instance, OpenActionResult, async_trait};
use serde_json::json;

pub struct PlaySoundboardSoundAction;

#[async_trait]
impl Action for PlaySoundboardSoundAction {
	const UUID: ActionUuid = "me.radsi.oadiscord.playsoundboardsound";
	type Settings = HashMap<String, String>;

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		let sound_id = match settings.get("soundId") {
			Some(v) => v.clone(),
			None => {
				instance.show_alert().await?;
				return Ok(());
			}
		};

		let guild_id = match settings.get("guildId") {
			Some(v) => v.clone(),
			None => {
				instance.show_alert().await?;
				return Ok(());
			}
		};

		let mut client_lock = discord_client().write().await;
		let Some(client) = client_lock.as_mut() else {
			log::error!("Discord client not initialized");
			instance.show_alert().await?;
			return Ok(());
		};

		let payload = json!({
			"cmd": "PLAY_SOUNDBOARD_SOUND",
			"args": {
				"sound_id": sound_id,
				"guild_id": guild_id,
			},
			"nonce": format!("play-soundboard-{}-{}", sound_id, guild_id),
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
			log::error!("Failed to play soundboard sound: {}", e);
			instance.show_alert().await?;
		}

		Ok(())
	}
}
