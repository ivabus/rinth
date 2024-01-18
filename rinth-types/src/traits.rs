use crate::file::Channel;

pub trait Synth {
	fn from_channel(channel: Channel, bpm: u16) -> Self
	where
		Self: Sized;

	fn synthesise(&self, sample_rate: u32) -> Vec<f32>;
}
