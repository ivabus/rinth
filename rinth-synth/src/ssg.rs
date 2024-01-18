use crate::shared::{add_delay, get_note};
use rinth_types::file::Channel;
use rinth_types::note::get_note_len;
use rinth_types::{file::ChannelType, note::Note, traits::Synth};
use std::f32::consts::PI;
use std::fs;

#[allow(clippy::upper_case_acronyms)]
pub struct SSG {
	pub bpm: u16,
	pub notes: Vec<Note>,
}

impl Synth for SSG {
	fn from_channel(channel: Channel, bpm: u16) -> Self
	where
		Self: Sized,
	{
		let channel = match channel {
			Channel {
				path,
				channel_type: ChannelType::SSG,
				..
			} => path,
			_ => {
				unreachable!()
			}
		};

		let contents = fs::read(channel).unwrap();

		let mut notes: Vec<Note> = vec![];
		for line in contents.split(|&x| x == b'\n') {
			if line.is_empty() {
				continue;
			}
			let s = String::from_utf8_lossy(line);
			let l = s.split_ascii_whitespace().collect::<Vec<&str>>();
			// Ignoring FM-only lines and comments
			if l[0].as_bytes()[0] == b'@'
				|| l[0].as_bytes()[0] == b'#'
				|| l[0].as_bytes()[0] == b'/'
			{
				continue;
			}
			notes.push(get_note(l))
		}
		SSG {
			bpm,
			notes,
		}
	}

	fn synthesise(&self, sample_rate: u32) -> Vec<f32> {
		// t - time, m - modulating freq (carrier)
		let y: fn(f32, f32) -> f32 = |t: f32, f: f32| (2_f32 * PI * f * t).sin().signum();
		let mut stream: Vec<f32> = vec![];
		for note in &self.notes {
			if note.delay != 0_f32 {
				add_delay(&mut stream, note.delay, self.bpm, sample_rate);
			}
			for k in 0..(get_note_len(note.length, self.bpm) * sample_rate as f32) as usize {
				stream.push(y(k as f32 / sample_rate as f32, note.tone.get_freq()))
			}
		}
		stream
	}
}
