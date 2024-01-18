//use rinth_macros::*;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Get real length of note from musical length
pub fn get_note_len(len: f32, bpm: u16) -> f32 {
	len * 240_f32 / bpm as f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
	pub tone: Tone,
	pub length: f32,
	pub delay: f32,
}

#[derive(Debug)]
pub enum ToneError {
	InvalidTone(String),
}

impl Display for ToneError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ToneError::InvalidTone(s) => format!("Invalid tone ({})", s),
			}
		)
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tone(String);

impl Tone {
	pub fn from(t: &str) -> Self {
		Tone(t.to_string())
	}

	pub fn get_freq(&self) -> f32 {
		const BASE_FREQ: f32 = 440.0;
		let sr = self.to_string();
		let modifier: i8 = if sr.as_bytes()[1] == b'b' {
			-1
		} else if sr.as_bytes()[1] == b'd' {
			1
		} else {
			0
		};
		let magic_num = (12 * sr.get(sr.len() - 1..).unwrap().parse::<i8>().unwrap() + modifier)
			as u8 + match sr.as_bytes()[0] {
			b'C' => 2,
			b'D' => 4,
			b'E' => 6,
			b'F' => 7,
			b'G' => 9,
			b'A' => 11,
			b'B' => 13,
			_ => 0,
		};
		BASE_FREQ * 2_f32.powf((magic_num as f32 - 59_f32) / 12_f32)
	}
}

impl Display for Tone {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.to_string())
	}
}
