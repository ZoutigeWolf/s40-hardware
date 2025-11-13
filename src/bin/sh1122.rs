use esp_hal::DriverMode;
use esp_hal::i2c::master::I2c;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::{Gray4, IntoStorage};
use embedded_graphics::prelude::OriginDimensions;

pub struct Sh1122<'a, T>
where
    T: DriverMode
{
    i2c: &'a mut I2c<'a, T>,
    addr: u8,
    buffer: [u8; 256 * 64 / 2],
}

impl<'a, T> Sh1122<'a, T>
where
    T: DriverMode
{
    pub fn new(i2c: &'a mut I2c<'a, T>, addr: u8) -> Self {
        Sh1122 {
            i2c,
            addr,
            buffer: [0; 256 * 64 / 2],
        }
    }

    pub fn init(&mut self) -> Result<(), ()> {
        let cmds = [
            0xAE,       // display off
            0xD5, 0x80, // set display clock divide ratio
            0xA8, 0x3F, // multiplex 64
            0xD3, 0x00, // display offset
            0x40,  // start line = 0
            0xAD, 0x8B, // DC-DC ON
            0xA0,       // segment mapping
            0xC8,       // COM scan direction
            0xDA, 0x12, // COM pins
            0x81, 0x7F, // contrast
            0xD9, 0xF1, // pre-charge
            0xDB, 0x40, // VCOM detect
            0xA4,       // display all on resume
            0xAF        // display on
        ];

        let mut i = 0;
        while i < cmds.len() {
            if i + 1 < cmds.len() {
                self.i2c.write(self.addr, &[0x00, cmds[i], cmds[i + 1]]).map_err(|_| ())?;
                i += 2;
            } else {
                self.i2c.write(self.addr, &[0x00, cmds[i]]).map_err(|_| ())?;
                i += 1;
            }
        }

        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x >= 256 || y >= 64 { return; }

        let value = if value > 15 { 15 } else { value };

        let y = 63 - y;
        let index = (y * 256 + x) / 2;
        let is_high_nibble = (x % 2) == 0;

        if is_high_nibble {
            self.buffer[index] = (self.buffer[index] & 0x0F) | (value << 4);
        } else {
            self.buffer[index] = (self.buffer[index] & 0xF0) | value;
        }
    }

    pub fn clear(&mut self) {
        for b in self.buffer.iter_mut() {
            *b = 0;
        }
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        for page in 0..8 {
            let page_addr = 0xB0 + page as u8;

            self.i2c.write(self.addr, &[0x00, page_addr]).map_err(|_| ())?;
            self.i2c.write(self.addr, &[0x00, 0x00]).map_err(|_| ())?;
            self.i2c.write(self.addr, &[0x00, 0x10]).map_err(|_| ())?;

            let start = page * 256 * 8 / 2;
            let end = start + 256 * 8 / 2;

            let mut page_data = [0u8; 1 + 256 * 8 / 2];
            page_data[0] = 0x40;
            page_data[1..].copy_from_slice(&self.buffer[start..end]);

            self.i2c.write(self.addr, &page_data).map_err(|_| ())?;
        }

        Ok(())
    }
}

impl<'a, T> DrawTarget for Sh1122<'a, T>
where
    T: DriverMode
{
    type Color = Gray4;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>
    {
        for Pixel(coord, color) in pixels {
            if let Ok((x @ 0..=255, y @ 0..=63)) = coord.try_into(){
                self.set_pixel(x as usize, y as usize, color.into_storage());
            }
        }
        Ok(())
    }
}

impl<'a, T> OriginDimensions for Sh1122<'a, T>
where
    T: DriverMode
{
    fn size(&self) -> Size {
        Size::new(256, 64)
    }
}