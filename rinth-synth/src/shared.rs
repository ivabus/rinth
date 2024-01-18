use rinth_types::note::{get_note_len, Note, Tone};

pub const DEFAULT_DELAY: f32 = 0.0;

#[inline(always)]
pub fn add_delay(stream: &mut Vec<f32>, delay: f32, bpm: u16, sample_rate: u32) {
	// println!("Pushing delay of len {}s", get_note_len(delay, bpm));
	stream.append(&mut vec![0_f32; (get_note_len(delay, bpm) * sample_rate as f32) as usize]);
}

pub fn get_note(l: Vec<&str>) -> Note {
	Note {
		tone: Tone::from(l[0]),
		length: match l[1].parse::<f32>() {
			Ok(f) => f,
			Err(_) => meval::eval_str(l[1]).unwrap() as f32,
		},
		delay: if l.len() > 2 {
			match l[2].parse::<f32>() {
				Ok(f) => f,
				Err(_) => meval::eval_str(l[2]).unwrap() as f32,
			}
		} else {
			DEFAULT_DELAY
		},
	}
}
