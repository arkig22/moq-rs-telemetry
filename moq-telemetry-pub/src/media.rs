use crate::cli::Config;
use anyhow::{self, Context, Ok};
use moq_transport::cache::{broadcast, segment, track};
use moq_transport::VarInt;
use std::time::{self, Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use bytes::Bytes;

pub struct Media {
	_broadcast: broadcast::Publisher,
	track: track::Publisher,
	sequence: u64,
}

impl Media {
	pub async fn new(_config: &Config, mut broadcast: broadcast::Publisher) -> anyhow::Result<Self> {

		//create the track for the telemetry
		let track = broadcast.create_track("telemetry")?;

		Ok(Media {
			_broadcast: broadcast,
			track: track,
			sequence: 1,
		})
	}

	pub async fn run(&mut self) -> anyhow::Result<()> {

		//wait ten seconds
		tokio::time::sleep(Duration::from_millis(10000)).await;

		loop {

			//get the current in millisec
			let now = SystemTime::now();
			let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
			let millisec = (since_the_epoch.as_millis() % 1_000_000_0) as i32;

			//create the timestamp vector and add the value
			let mut timestamps = Vec::new();
			timestamps.push(millisec);

			//starting hop count
			let hops = 1;

			// Create a new segment.
			let mut segment = self.track.create_segment(segment::Info {
				sequence: VarInt::try_from(self.sequence).context("sequence too large")?,
				hops,
				priority: timestamps.clone(), //I use the priority header fot the timestamp, because both will fit in a i32

				// Delete segments after 10s.
				expires: Some(time::Duration::from_secs(10)),
			})?;

			self.sequence += 1;

			//sending 50 bytes
			let ones: String = "1".repeat(50);
			let data = Bytes::from(ones);

			segment.write_chunk(data)?;

			//wait one second
			tokio::time::sleep(Duration::from_millis(1000)).await;

			//quit after 60 seconds
			if self.sequence > 60{
				log::info!("finished.");
				break Ok(());
			}
		}
	}
}
