extern mod png;

struct Pixel(u8, u8, u8);

fn to_vec(pixels: ~[Pixel]) -> ~[u8] {
  pixels.flat_map(|&Pixel(r, g, b)| { ~[r, g, b] })
}

fn main() {
  let pixels = ~[Pixel(0, 0, 255), ..(100 * 100)];

  let img = png::Image {
    width: 100,
    height: 100,
    color_type: png::RGB8,
    pixels: to_vec(pixels)
  };

  png::store_png(&img, &Path::new("foo.png"));
}
