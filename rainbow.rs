extern mod png;

use std::iter::count;

struct Pixel(u8, u8, u8);

fn to_vec(pixels: ~[Pixel]) -> ~[u8] {
  pixels.flat_map(|&Pixel(r, g, b)| { ~[r, g, b] })
}

fn main() {
  let mut line = ~[Pixel(0, 0, 0), ..100];

  for x in range(0u, 100) {
    line[x] = match x {
      x if x < 25 => { Pixel(255, count(0u8, 255 / 25).nth(x % 25).unwrap(), 0) }
      x if x < 50 => { Pixel(255 - count(1u8, 255 / 25).nth(x % 25).unwrap(), 255, 0) }
      x if x < 75 => { Pixel(0, 255 - count(1u8, 255 / 25).nth(x % 25).unwrap(), count(0u8, 255 / 25).nth(x % 25).unwrap()) }
      _ => { Pixel(0, count(0u8, 192 / 25).nth(x % 25).unwrap(), 255) }
    }
  }

  let pixels = std::vec::from_fn(100 * 100, |i| { line[i % 100] });

  let img = png::Image {
    width: 100,
    height: 100,
    color_type: png::RGB8,
    pixels: to_vec(pixels)
  };

  png::store_png(&img, &Path::new("rainbow.png"));
}
