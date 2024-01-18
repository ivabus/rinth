use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub enum FileType {
	Header(Header),
	Channel(Channel),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ChannelType {
	FM,
	SSG,
}

#[derive(Deserialize, Serialize)]
pub struct Header {
	pub name: String,
	pub bpm: u16,
	pub channels: Vec<Channel>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Channel {
	pub path: PathBuf,
	#[serde(alias = "type")]
	pub channel_type: ChannelType,
	pub volume: Option<f32>,
}
