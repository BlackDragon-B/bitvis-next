use std::io::prelude::*;
use std::process::{Child, ChildStdin, Command, Stdio};
use crate::renderer::{Color, Pixel, ColorTypes};

fn flatten<T, const N: usize>(v: Vec<Vec<[T; N]>>) -> Vec<T> {
    v.into_iter().flatten().flatten().collect()
}

pub struct Renderer {
    pub ffmpeg: Child,
    pub framebuffer: Vec<Pixel>,
    pub size: (usize, usize),
}

impl Renderer {
    pub fn spawn(w: usize, h: usize, framerate: usize) -> Renderer {
        let child = Command::new("ffplay")
        .arg("-f")
        .arg("rawvideo")
        // ... where every four bytes are [r, g, b, a] format
        .arg("-pixel_format")
        .arg("rgb24")
        // The size of the video is 3840x2160
        .arg("-video_size")
        .arg(format!("{}x{}",w,h))
        // 60 frames per second
        .arg("-framerate")
        .arg(framerate.to_string())
        // Get the data from stdin
        .arg("-")
        // stdin, stderr, and stdout are piped
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // Run the child command
        .spawn()
        .unwrap();
        println!("spawn");
        return Renderer {ffmpeg: child, framebuffer: vec![Pixel {red: 0, green: 0, blue: 0}; w*h], size: (w, h) }
    }
    pub fn write_framebuffer(&mut self) {
        let stdin = self.ffmpeg.stdin.as_mut().unwrap();
        let flattened: Vec<u8> = <Vec<Pixel> as Clone>::clone(&self.framebuffer).into_iter().flatten().collect();
        let _ = stdin.write_all(&flattened);
    }
    pub fn update_framebuffer(&mut self, x: usize, y: usize, pixel: ColorTypes) {
        let p = match pixel {
            ColorTypes::Color(c) => {
                let r = match c {
                    Color::Blank => Pixel { red: 0, green: 0, blue: 0 },
                    Color::Red => Pixel { red: 255, green: 0, blue: 0 },
                    Color::Orange => Pixel { red: 255, green: 200, blue: 0 },
                    Color::Green => Pixel { red: 0, green: 255, blue: 0 },
                };
                r
            },
            ColorTypes::Pixel(c) => c // TODO: do proper RGB to Color parsing,
        };
        if x <= self.size.1.try_into().unwrap() && y <= self.size.0.try_into().unwrap() && y > 0 && x > 0 {
            self.framebuffer[(x as usize-1)*self.size.0+(y as usize-1) as usize] = p
        }
    }
    pub fn clear_framebuffer(&mut self) {
        for p in self.framebuffer.iter_mut() {
            *p = Pixel {red: 0, green: 0, blue: 0}
        };
    }
}
