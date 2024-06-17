mod renderer;
mod audio;
mod mprishandler;
use std::f32::consts::E;
use std::io::Write;
use std::{thread, time};
use renderer::ffmpeg;

use crate::renderer::ffmpeg::Pixel;

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
    let mut child = ffmpeg::FfmpegRenderer::spawn(120, 48, 30);
    let mut e = 0;
    // thread::spawn(|| {
    //     audio::start();
    // });

    loop {
        let text: &str = "";
        let x = m.get_metadata();
        let metadata: &str = match &x {
            Ok(T) => T.title().unwrap(),
            Err(er) => {
                m.get_player();
                er
            },
        };
        let mut relativepos: i32 = -1;
        let test: Vec<char> = metadata.chars().collect();
        for c in test {
            let ch = font.glyphs().get(&c).unwrap();
            let pixels = ch.pixels();
            for p in pixels {
                //println!("{:?}", p);
                if p.1 == true {
                    child.update_framebuffer(p.0.1 as usize+1+(child.size.1-ch.height() as usize), p.0.0 as usize+1+relativepos as usize, Pixel { red: 255, green: 255, blue: 255 })
                }
            }
            relativepos = relativepos + ch.width() as i32
        }
        child.write_framebuffer();
        thread::sleep(time::Duration::from_millis(8));
        child.clear_framebuffer()
    }

    // let output = child.wait_with_output().unwrap();
    // println!("{}", String::from_utf8(output.stdout).unwrap());
    // println!("{}", String::from_utf8(output.stderr).unwrap());
    // println!("Generated {} frames.", frames)

}