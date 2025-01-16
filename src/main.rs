mod renderer;
mod audio;
mod mprishandler;
use std::f32::consts::E;
use std::io::Write;
use std::time::Duration;
use std::{thread, time};
use audio::ADev;
use audioviz::spectrum::stream;
use mpris::Metadata;
use renderer::ffmpeg;
use renderer::panel;

use crate::renderer::Pixel;
use crate::renderer::Color;
use crate::renderer::ColorTypes;

use audioviz::io::{Device, Input, InputController};
use audioviz::spectrum::{
    config::{Interpolation, ProcessorConfig, StreamConfig},
    stream::Stream,
    Frequency,
};

fn main() {
    //mpris test
    let mut m = match mprishandler::create() {
        Ok(T) => T,
        Err(er) => { eprintln!("{:?}",er); std::process::exit(1) }
    };
    m.get_player();
    //font test
    
    let font = bdf::open("font.bdf").unwrap();

    //font test end
    println!("Hello, world!");
    let mut child = panel::Renderer::spawn(120, 48, 30);
    let mut e = 0;
    // thread::spawn(|| {
    //     audio::start();
    // });
    let mut relativeposback: i32 = 0;
    // let mut aud = ADev::start();
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
            resolution: Some(120),
            ..ProcessorConfig::default()
        },
        ..StreamConfig::default()
    };
    let mut stream = Stream::new(stream_config);
    loop {
        //audiodingen ofzo
        // let a = aud.get_spectrum();
        if let Some(new_data) = audio_receiver.pull_data() {
            stream.push_data(new_data);
        }

        stream.update();

        let frequencies: Vec<Vec<Frequency>> = stream.get_frequencies();
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
        //erboven uitsnijden en in module proppen indien mogelijk
        let mut pos = 0;
        for freq in frequencies {
            let mut f = (((1.0-freq.volume)*48 as f32) as isize)-10;
            if f >= 0 {
                for i in f..38 {
                    child.update_framebuffer(i as usize, pos, ColorTypes::Color(Color::Green));
                }
            }
            pos = pos+1;
        }
        //tekstdingen ofzo
        let text: &str = "";
        let x = m.get_metadata();
        let metadata: (Option<&Metadata>, Option<&str>) = match &x {
            Ok(T) => (Some(T),None),
            Err(er) => {
                m.get_player();
                (None,Some(er))
            },
        };

        let f = match metadata.0 {
            Some(mdata) => {
                let title = match mdata.title() {
                    Some(title) => title,
                    None => "No Title",
                };
                let artist = match mdata.artists() {
                    Some(artist) => { artist.join(", ")},
                    None => "No Artist".to_string(),
                };
                let duration = match mdata.length() {
                    Some(D) => D,
                    None => Duration::from_secs(0),
                };
                (format!("{} - {}", title, artist), duration)    
            },
            None => ("No song playing".to_string(), Duration::from_secs(0)),
        };

        //progressbar renderer
        let progress = match m.get_progress() {
            Ok(T) => T,
            Err(_) => {println!("hi"); Duration::from_secs(0)},
        };

        let length = f.1;

        let remainder: f32 = progress.as_secs_f32() / length.as_secs_f32();
        //println!("{:?} {:?}", progress.as_secs_f32(), length.as_secs_f32());
        //println!("{:?}", remainder);
        let l = (child.size.0 as f32*remainder) as u16;
        //println!("l {:?}",l);
        for i in 0..l {
            child.update_framebuffer(child.size.1-10, i as usize, ColorTypes::Color(Color::Green))
        }
        for i in 0+l..(child.size.0+1) as u16{
            child.update_framebuffer(child.size.1-10, i as usize, ColorTypes::Color(Color::Orange))
        }

        //text renderer
        let mut relativepos: u32 = 0;
        let test: Vec<char> = f.0.chars().collect();
        let mut totalwidth: i32 = 0;
        for c in &test {
            totalwidth = totalwidth + match font.glyphs().get(&c) {
                Some(a) => a.width() as i32,
                None => 0 as i32,
            }
        }    
        for c in test {
            let ch = match font.glyphs().get(&c) {
                Some(ch) => ch,
                None => {continue;},
            };
            let pixels = ch.pixels();
            for p in pixels {
                //println!("{:?}", p);
                if p.1 == true {
                    let y = (p.0.0 as usize+1+relativepos as usize) as i32 - relativeposback as i32;
                    if y >= 0 {
                        child.update_framebuffer(p.0.1 as usize+1+(child.size.1-ch.height() as usize), y as usize, ColorTypes::Color(Color::Green))
                    } 
                }
            }
            relativepos = relativepos + ch.width()
        }
        if totalwidth > child.size.0.try_into().unwrap() {
            if relativeposback > totalwidth {
                relativeposback = 0 - (child.size.0 as i32);
            } else {
                relativeposback += 1;
            }
        } else {
            relativeposback = 0;
        };
        child.write_framebuffer();
        thread::sleep(time::Duration::from_millis(32));
        child.clear_framebuffer()
    }

    // let output = child.wait_with_output().unwrap();
    // println!("{}", String::from_utf8(output.stdout).unwrap());
    // println!("{}", String::from_utf8(output.stderr).unwrap());
    // println!("Generated {} frames.", frames)

}