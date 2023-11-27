use crate::cli::Config;
use anyhow::{self};
use moq_transport::{cache::{broadcast, track}, VarInt};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

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
			let segment  = self.track.next_segment().await.unwrap().unwrap();
			log::info!("{:?}, {:?}", segment.sequence, segment.priority);

			let formatted_text = format!("{:?}, {:?}\n", segment.sequence, segment.priority);
			//let mut file = File::create("p-r-s_minikube.txt").await?;

			let mut file = OpenOptions::new()
        		.create(true)
        		.append(true)
        		.open("p-r-r-s_remote_central2_west3_mcs.txt").await?;

    		file.write_all(formatted_text.as_bytes()).await?;

			if segment.sequence == VarInt::from_u32(60){
				log::info!("finished.");
				break Ok(());
			}
		}
	}
}
