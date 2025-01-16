use std::io::prelude::*;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::{io::Write, net::{SocketAddr, TcpStream}};
use crate::renderer::{Color, Pixel, ColorTypes};

fn flatten<T, const N: usize>(v: Vec<Vec<[T; N]>>) -> Vec<T> {
    v.into_iter().flatten().flatten().collect()
}

pub struct Renderer {
    pub socket: TcpStream,
    pub framebuffer: Vec<Color>,
    pub size: (usize, usize),
}

impl Renderer {
    pub fn spawn(w: usize, h: usize, framerate: usize) -> Renderer {
        let socket = TcpStream::connect("100.64.0.171:1337").unwrap();
        println!("spawn");
        return Renderer {socket, framebuffer: vec![Color::Blank; w*h], size: (w, h) }
    }   
    pub fn write_framebuffer(&mut self) {
        //println!("{:?}",&self.framebuffer);
        let mut m: Vec<bool> = Vec::new();
        for c in &self.framebuffer {
            match c {
                Color::Blank => {m.append(&mut vec![false, false])},
                Color::Red => {m.append(&mut vec![true, false])},
                Color::Orange => {m.append(&mut vec![true, true])},
                Color::Green => {m.append(&mut vec![false, true])},
            }
            //println!("{:?}", m);
        }
        let mut bytes: Vec<u8> = Vec::new();
        for z in m.chunks(8) {
            bytes.push(z.iter().fold(0u8, |v, b| (v << 1) + (*b as u8)));
        };
        let mut byt2es = vec![0x3a, 0x30, 0x30];

        self.socket.write(&vec![byt2es,bytes].concat());
    }
    pub fn update_framebuffer(&mut self, x: usize, y: usize, pixel: ColorTypes) {
        let p = match pixel {
            ColorTypes::Color(c) => c,
            ColorTypes::Pixel(c) => {Color::Red} // TODO: do proper RGB to Color parsing,
        };
        if x <= self.size.1.try_into().unwrap() && y <= self.size.0.try_into().unwrap() && y > 0 && x > 0 {
            self.framebuffer[(x as usize-1)*self.size.0+(y as usize-1) as usize] = p
        }
    }
    pub fn clear_framebuffer(&mut self) {
        for p in self.framebuffer.iter_mut() {
            *p = Color::Blank
        };
    }
}
