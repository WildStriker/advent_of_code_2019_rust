use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(PartialEq, Eq, Hash)]
pub enum Pixels {
    White,
    Black,
    Transparent,
}

impl Pixels {
    fn new(c: char) -> Result<Self, String> {
        let d = c
            .to_digit(10)
            .ok_or_else(|| "Unable to convert character to a digit!")?;

        match d {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            2 => Ok(Self::Transparent),
            x => Err(format!("Unexpected Pixel Type: {}", x)),
        }
    }
}

struct Layer<'a> {
    pixels: &'a [Pixels],
}

impl<'a> Layer<'a> {
    fn count(&self) -> HashMap<&Pixels, usize> {
        let mut counter = HashMap::new();
        for pixel in self.pixels {
            match counter.get_mut(pixel) {
                Some(v) => *v += 1,
                None => {
                    counter.insert(pixel, 1);
                }
            }
        }
        counter
    }
}

pub struct Image<'a> {
    layers: Vec<Layer<'a>>,
    wide: usize,
    tall: usize,
    pixels_per_layer: usize,
}

impl<'a> Image<'a> {
    pub fn new(pixels: &'a Vec<Pixels>, wide: usize, tall: usize) -> Result<Self, String> {
        let mut layers = Vec::new();
        let pixels_per_layer = wide * tall;

        if pixels.len() % pixels_per_layer != 0 {
            return Err("There are some pixels remaining, unable to generate image".to_string());
        }

        for start in (0..pixels.len()).step_by(pixels_per_layer) {
            let layer = Layer {
                pixels: &pixels[start..start + pixels_per_layer],
            };
            layers.push(layer);
        }

        Ok(Self {
            layers,
            wide,
            tall,
            pixels_per_layer,
        })
    }

    pub fn validate(&self) -> usize {
        let mut fewest = 0;
        let mut fewest_counter = None;
        for layer in &self.layers {
            let counter = layer.count();

            if let Some(count) = counter.get(&Pixels::Black) {
                match fewest_counter {
                    Some(_) if count >= &fewest => (),
                    _ => {
                        fewest = *count;
                        fewest_counter = Some(counter);
                    }
                }
            }
        }
        match fewest_counter {
            Some(counter) => {
                let white_count = match counter.get(&Pixels::White) {
                    Some(count) => count,
                    None => &0,
                };
                let transparent_count = match counter.get(&Pixels::Transparent) {
                    Some(count) => count,
                    None => &0,
                };

                white_count * transparent_count
            }
            None => 0,
        }
    }

    /// flatten pixel working our way down the layers stack
    /// continues until a non-transparent pixel is found
    /// or no more stacks are left and defaults to black
    fn flatten_pixel(&self, layer_index: usize, pixel_index: usize) -> &Pixels {
        match &self.layers[layer_index].pixels[pixel_index] {
            Pixels::Transparent => {
                let next_layer = layer_index + 1;
                if next_layer >= self.layers.len() {
                    // last layer reach, default to black
                    return &Pixels::Black;
                }
                self.flatten_pixel(next_layer, pixel_index)
            }
            pixel => pixel,
        }
    }

    /// Renders an image to a visible string "image"
    pub fn render(&self) -> String {
        let mut display = String::with_capacity(self.pixels_per_layer + self.tall);

        for pixel_index in 0..self.pixels_per_layer {
            if pixel_index % self.wide == 0 {
                display.push_str("\n");
            }
            display.push_str(match self.flatten_pixel(0, pixel_index) {
                Pixels::Black | Pixels::Transparent => " ",
                Pixels::White => "*",
            });
        }
        display
    }
}

impl<'a> fmt::Display for Image<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

pub fn parse<T>(reader: T) -> Result<Vec<Pixels>, String>
where
    T: io::BufRead,
{
    reader
        .lines()
        .next()
        .ok_or_else(|| "Unable to parse! Empty File?")?
        .or_else(|x| Err(format!("Error Parsing File: {}", x)))?
        .chars()
        .map(|c| Pixels::new(c))
        .collect::<Result<Vec<Pixels>, String>>()
}
