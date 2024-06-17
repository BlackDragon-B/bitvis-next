use std::io::prelude::*;
use std::process::{Child, ChildStdin, Command, Stdio};

#[derive(Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl IntoIterator for Pixel {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.red, self.green, self.blue])
    }
}

fn flatten<T, const N: usize>(v: Vec<Vec<[T; N]>>) -> Vec<T> {
    v.into_iter().flatten().flatten().collect()
}

pub struct FfmpegRenderer {
    pub ffmpeg: Child,
    pub framebuffer: Vec<Pixel>,
    pub size: (usize, usize),
}

impl FfmpegRenderer {
    pub fn spawn(w: usize, h: usize, framerate: usize) -> FfmpegRenderer {
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
        return FfmpegRenderer {ffmpeg: child, framebuffer: vec![Pixel {red: 0, green: 0, blue: 0}; w*h], size: (w, h) }
    }
    pub fn write_framebuffer(&mut self) {
        let stdin = self.ffmpeg.stdin.as_mut().unwrap();
        let flattened: Vec<u8> = <Vec<Pixel> as Clone>::clone(&self.framebuffer).into_iter().flatten().collect();
        let _ = stdin.write_all(&flattened);
    }
    pub fn update_framebuffer(&mut self, x: isize, y: isize, pixel: Pixel) {
        if x <= self.size.1.try_into().unwrap() && y <= self.size.0.try_into().unwrap() && y > 0 && x > 0 {
            self.framebuffer[(x as usize-1)*self.size.0+(y as usize-1) as usize] = pixel
        }
    }
    pub fn clear_framebuffer(&mut self) {
        for p in self.framebuffer.iter_mut() {
            *p = Pixel {red: 0, green: 0, blue: 0}
        };
    }
}
