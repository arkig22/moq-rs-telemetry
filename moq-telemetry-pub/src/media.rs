use crate::cli::Config;
use anyhow::{self, Context, Ok};
use moq_transport::cache::{broadcast, segment, track};
use moq_transport::VarInt;
use std::time;
use std::time::{SystemTime, UNIX_EPOCH};
use bytes::Bytes;
use std::thread;
use std::time::Duration;


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

		loop {

			//get the current in millisec
			let now = SystemTime::now();
			let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
			let millisec = (since_the_epoch.as_millis() % 1_000_000_0) as i32;

			// Create a new segment.
			let mut segment = self.track.create_segment(segment::Info {
				sequence: VarInt::try_from(self.sequence).context("sequence too large")?,
				priority: millisec, //I use the priority header fot the timestamp, because both will fit in a i32

				// Delete segments after 10s.
				expires: Some(time::Duration::from_secs(10)),
			})?;

			self.sequence += 1;

			//just sending some random bites
			let data = Bytes::from("111111");
			segment.write_chunk(data)?;

			//log::debug!("segment: {:?}", segment);

			//wait one second
    		tokio::time::sleep(Duration::from_millis(1000)).await;
		}
	}
}
