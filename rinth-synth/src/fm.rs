use crate::shared::{add_delay, get_note};
use rinth_types::file::Channel;
use rinth_types::note::get_note_len;
use rinth_types::{file::ChannelType, note::Note, traits::Synth};
use std::f32::consts::PI;
use std::fs;

pub struct FM {
	pub bpm: u16,
	pub notes: Vec<(Note, ModulationFreq, FrequencyDeviation)>,
}

#[derive(Copy, Clone)]
pub struct ModulationFreq(f32);

#[derive(Copy, Clone)]
pub struct FrequencyDeviation(f32);

const FIGHT_CLICKS: usize = 128;

impl Synth for FM {
	fn from_channel(channel: Channel, bpm: u16) -> Self
	where
		Self: Sized,
	{
		let channel = match channel {
			Channel {
				path,
				channel_type: ChannelType::FM,
				..
			} => path,
			_ => {
				unreachable!()
			}
		};

		let contents = fs::read(channel).unwrap();

		let mut notes: Vec<(Note, ModulationFreq, FrequencyDeviation)> = vec![];
		let mut current_tone = ModulationFreq(440.0);
		let mut current_deviation = FrequencyDeviation(8.0);

		for line in contents.split(|&x| x == b'\n') {
			if line.is_empty() {
				continue;
			}
			let s = String::from_utf8_lossy(line);
			let l = s.split_ascii_whitespace().collect::<Vec<&str>>();
			// Ignore comment
			if l[0].as_bytes()[0] == b'/' {
				continue;
			}
			// String in FM channel may be or carrier tone or note
			if l[0].as_bytes()[0] == b'@' {
				current_tone = ModulationFreq(l[0][1..].parse::<f32>().unwrap());
				continue;
			}
			if l[0].as_bytes()[0] == b'#' {
				current_deviation = FrequencyDeviation(l[0][1..].parse::<f32>().unwrap());
				continue;
			}
			notes.push((get_note(l), current_tone, current_deviation))
		}
		FM {
			bpm,
			notes,
		}
	}

	fn synthesise(&self, sample_rate: u32) -> Vec<f32> {
		// t - time, m - modulating freq (carrier)
		let y: fn(f32, f32, f32, f32) -> f32 =
			|t: f32, f: f32, m: f32, d| (2_f32 * PI * m * t + d * (2_f32 * PI * f * t).sin()).cos();
		let mut stream: Vec<f32> = vec![];
		for (note, tone, deviation) in &self.notes {
			if note.delay != 0_f32 {
				add_delay(&mut stream, note.delay, self.bpm, sample_rate);
			}
			// Samples in _THIS_ note
			let samples = (get_note_len(note.length, self.bpm) * sample_rate as f32) as usize;
			for k in 0..samples {
				stream.push(
					y(k as f32 / sample_rate as f32, tone.0, note.tone.get_freq(), deviation.0)
						// This makes a little linear fade-in-out so we don't get "clicks"
						* if k <= FIGHT_CLICKS {
							k as f32 / FIGHT_CLICKS as f32
					} else if k
							>= samples
								- FIGHT_CLICKS
						{
							(samples
								- k) as f32 / FIGHT_CLICKS as f32
						} else {
							1.0
						},
				)
			}
		}
		stream
	}
}
