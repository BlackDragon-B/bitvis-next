pub mod ffmpeg;
pub mod panel;



#[derive(Clone)]
pub enum Color {
    Blank,
    Red,
    Orange,
    Green,
}

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

pub enum ColorTypes {
    Color(Color),
    Pixel(Pixel)
}