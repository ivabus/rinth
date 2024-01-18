use clap::Parser;
use rinth_types::file::ChannelType;
use rinth_types::traits::Synth;
use std::cmp::max;
use std::io::Read;
use std::path::PathBuf;

mod fm;
mod shared;
mod ssg;

#[derive(Parser)]
enum Commands {
	#[command(visible_alias = "b")]
	Build(Build),
	#[command(visible_alias = "m")]
	Master(Build),
}

#[derive(Parser)]
#[command(author, about, version)]
struct Build {
	path: PathBuf,
	#[arg(short, long, default_value = "44100")]
	sample_rate: u32,
}

fn truncate(sample: f32) -> f32 {
	if sample.abs() > 1_f32 {
		sample.signum()
	} else {
		sample
	}
}

fn merge(master: &mut Vec<f32>, slave: Vec<f32>) {
	master.resize(max(master.len(), slave.len()), 0_f32);
	let slave_len = slave.len();
	for (n, sample) in &mut master.iter_mut().enumerate() {
		*sample = if n < slave_len {
			truncate(*sample + slave[n])
		} else {
			truncate(*sample)
		}
	}
}

fn main() {
	let args = Commands::parse();

	let (args, mastering) = match args {
		Commands::Build(args) => (args, false),
		Commands::Master(args) => (args, true),
	};
	let mut project_file = std::fs::File::open(&args.path).unwrap();
	let mut project = String::new();
	project_file.read_to_string(&mut project).unwrap();

	let project: rinth_types::file::Header = serde_yaml::from_str(&project).unwrap();
	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: args.sample_rate,
		bits_per_sample: 32,
		sample_format: hound::SampleFormat::Float,
	};
	let mut master = vec![];
	for channel in project.channels {
		let mut channel = channel;
		let mut channel_path = args.path.parent().unwrap().to_path_buf();
		channel_path.push(channel.path);
		channel.path = channel_path;

		let synth: Box<dyn Synth> = match channel.channel_type {
			ChannelType::FM => Box::new(fm::FM::from_channel(channel.clone(), project.bpm)),
			ChannelType::SSG => Box::new(ssg::SSG::from_channel(channel.clone(), project.bpm)),
		};
		if !mastering {
			let mut writer = hound::WavWriter::create(
				format!(
					"{}-{:?}.wav",
					channel.path.as_os_str().to_str().unwrap(),
					channel.channel_type
				),
				spec,
			)
			.unwrap();
			synth.synthesise(args.sample_rate).iter().for_each(|sample| {
				writer
					.write_sample(if let Some(volume) = channel.volume {
						*sample * volume
					} else {
						*sample
					})
					.unwrap()
			});
			writer.finalize().unwrap();
		} else {
			merge(
				&mut master,
				synth
					.synthesise(args.sample_rate)
					.iter()
					.map(|sample| {
						if let Some(volume) = channel.volume {
							*sample * volume
						} else {
							*sample
						}
					})
					.collect(),
			);
		}
	}
	if mastering {
		let mut writer = hound::WavWriter::create(
			format!(
				"{}/{}-master.wav",
				args.path.parent().unwrap().to_path_buf().to_str().unwrap(),
				project.name
			),
			spec,
		)
		.unwrap();
		master.iter().for_each(|sample| writer.write_sample(*sample).unwrap());
		writer.finalize().unwrap();
	}
}
