use crate::cli::Config;
use anyhow::{self};
use moq_transport::cache::{broadcast, track};

pub struct Media {
	_broadcast: broadcast::Subscriber,
	track: track::Subscriber
}

impl Media {
	pub async fn new(_config: &Config, broadcast: broadcast::Subscriber) -> anyhow::Result<Self> {

		let track = broadcast.get_track("telemetry")?;

		Ok(Media {
			_broadcast: broadcast,
			track: track,
		})
	}

	pub async fn run(&mut self) -> anyhow::Result<()> {

		loop {
			let _segment  = self.track.next_segment().await;
		}
	}
}
