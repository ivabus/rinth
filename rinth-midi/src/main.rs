use clap::Parser;
use midly::num::{u28, u7};
use midly::{MidiMessage, Smf, TrackEventKind};
use rinth_types::note::{Note, Tone};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Args {
	midi: PathBuf,

	#[arg(short, default_value = "0")]
	channel: u8,
}
fn main() {
	let args = Args::parse();
	let binding = std::fs::read(args.midi).unwrap();
	let smf = Smf::parse(binding.as_slice()).unwrap();
	let mut is_pressed = false;
	let mut delay = 0;
	let mut note_pressed = u7::new(0);
	let mut timer = u28::new(0);
	let mut notes: Vec<Note> = vec![];
	let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
	for track in smf.tracks.iter() {
		for event in track {
			if let TrackEventKind::Midi {
				channel: _channel,
				message,
			} = event.kind
			{
				match message {
					MidiMessage::NoteOff {
						key,
						..
					} => {
						if is_pressed && note_pressed == key {
							let note = Note {
								tone: Tone::from(&format!(
									"{}{}",
									note_names[(key.as_int() % 12) as usize],
									(key.as_int() / 12) - 1
								)),
								length: event.delta.as_int() as f32 / 1000.0,
								delay: delay as f32 / 1000.0,
							};
							notes.push(note);
							note_pressed = u7::new(0);
							is_pressed = false;
						}
					}
					MidiMessage::NoteOn {
						key,
						..
					} => {
						if !is_pressed {
							eprintln!("pressing on {}", key);
							is_pressed = true;
							note_pressed = key;
							timer = u28::new(0);
							delay = event.delta.as_int();
						}
					}
					_ => {}
				}
			}
			timer += event.delta;
		}
	}
	for note in notes {
		println!("{} {} {}", note.tone, note.length, note.delay);
	}
}
