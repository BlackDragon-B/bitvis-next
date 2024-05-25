mod renderer;
mod audio;
use std::io::Write;
use std::{thread, time};
use renderer::ffmpeg;
use mpris::PlayerFinder;

use crate::renderer::ffmpeg::Pixel;

fn main() {
    //mpris test
    let player = PlayerFinder::new()
    .expect("Could not connect to D-Bus")
    .find_active()
    .expect("Could not find any player");

    player.pause().expect("Could not pause");

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
        let mut relativepos: u32 = 0;
        let metadata = player.get_metadata().expect("Could not get metadata for player");
        let test: Vec<char> = format!("{} - {}", metadata.title().unwrap(), metadata.artists().unwrap()[0]).chars().collect();
        for c in test {
            let ch = font.glyphs().get(&c).unwrap();
            let pixels = ch.pixels();
            for p in pixels {
                //println!("{:?}", p);
                if p.1 == true {
                    child.update_framebuffer(p.0.1 as usize+1, p.0.0 as usize+1+relativepos as usize, Pixel { red: 255, green: 255, blue: 255 })
                }
            }
            relativepos = relativepos + ch.width();
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