use std::thread;

use audioviz::io::{Device, Input, InputController};
use audioviz::spectrum::{
    config::{Interpolation, ProcessorConfig, StreamConfig},
    stream::Stream,
    Frequency,
};

pub struct ADev {
    stream: Stream,
    audio_receiver: InputController,
    channel_count: u16,
}
impl ADev {
    pub fn start() -> ADev {
        let mut audio_input = Input::new();
        let devices = audio_input.fetch_devices().unwrap();
        for (id, device) in devices.iter().enumerate() {
            println!("{id}\t{device}");
        }
        let (channel_count, _sampling_rate, audio_receiver) = audio_input.init(&Device::Id(4), Some(1024)).expect("AAA");

        let stream_config: StreamConfig = StreamConfig {
            channel_count: 1,
            gravity: Some(2.0),
            fft_resolution: 1024 * 3,
            processor: ProcessorConfig {
                frequency_bounds: [35, 20_000],
                interpolation: Interpolation::Cubic,
                volume: 0.4,
                resolution: None,
                ..ProcessorConfig::default()
            },
            ..StreamConfig::default()
        };
        println!("CC {:?}",channel_count);
        return ADev { stream: Stream::new(stream_config), audio_receiver, channel_count }
    }
    pub fn get_spectrum(&mut self) -> Vec<Frequency> {
        if let Some(new_data) = self.audio_receiver.pull_data() {
            self.stream.push_data(new_data);
        }

        self.stream.update();

        let frequencies: Vec<Vec<Frequency>> = self.stream.get_frequencies();
        let frequencies: Vec<Frequency> = if frequencies.len() >= 2 {
            let mut buf: Vec<Frequency> = Vec::new();
            // left
            let mut left = frequencies[0].clone();
            left.reverse();
            buf.append(&mut left);
            // right
            buf.append(&mut frequencies[1].clone());
            buf
        } else {
            if frequencies.len() == 1 {
                frequencies[0].clone()
            } else {
                Vec::new()
            }
        };
        return frequencies;
    }//HAHA MY ASS
}