extern mod png;

use std::num::{sqrt, min, max, cos, sin};

static WIDTH: f64 = 800.;
static HEIGHT: f64 = 800.;

struct Color {
  r: u8,
  g: u8,
  b: u8
}

static Red: Color = Color { r: 255, g: 0, b: 0 };
static Green: Color = Color { r: 0, g: 255, b: 0 };
static Blue: Color = Color { r: 0, g: 0, b: 255 };
static White: Color = Color { r: 255, g: 255, b: 255 };
static Black: Color = Color { r: 0, g: 0, b: 0 };

struct Point {
  x: f64,
  y: f64,
  z: f64
}

struct Spot {
  pos: Point,
  color: Color,
}

struct Sphere {
  pos: Point,
  radius: f64,
  shininess: f64,
  color: Color
}

struct Scene {
  eye: Point,
  spot: Spot,
  sphere: Sphere
}

fn solve_poly(a: f64, b: f64, c: f64) -> f64 {
  let delta = b.pow(&2.0) - 4. * a * c;

  if delta < 0. {
    return 0.;
  }

  let k1 = (-b - sqrt(delta)) / (2. * a);
  let k2 = (-b + sqrt(delta)) / (2. * a);

  return match (k1, k2) {
    (k1, k2) if k1 < 0. && k2 < 0. => 0.,
    (k1, k2) if k2 <= 0. => k1,
    (k1, k2) if k1 <= 0. => k2,
    _ => min(k1, k2)
  }
}

fn hit_sphere(sphere: &Sphere, eye: &Point, vector: &Point) -> f64 {
  let a = vector.x.pow(&2.) + vector.y.pow(&2.) + vector.z.pow(&2.);
  let b = 2. * (eye.x * vector.x + eye.y * vector.y + eye.z * vector.z);
  let c = eye.x.pow(&2.) + eye.y.pow(&2.) + eye.z.pow(&2.) - sphere.radius.pow(&2.);
  return solve_poly(a, b, c);
}

fn get_closest(obj: &Sphere, eye: &Point, vector: &Point) -> f64 {
  let e = ~Point { x: eye.x - obj.pos.x, y: eye.y - obj.pos.y, z: eye.z - obj.pos.z };
  let v = ~Point { x: vector.x, y: vector.y, z: vector.z };
  return hit_sphere(obj, e, v);
}

fn get_light(obj: &Sphere, spot: &Spot, inter: &Point, light: &Point) -> Color {
  let perp = Point { x: inter.x, y: inter.y, z: inter.z };
  let norme_l = sqrt(light.x.pow(&2.) + light.y.pow(&2.) + light.z.pow(&2.));
  let norme_n = sqrt(perp.x.pow(&2.) + perp.y.pow(&2.) + perp.z.pow(&2.));
  let cos_a = (light.x * perp.x + light.y * perp.y + light.z * perp.z) / (norme_l * norme_n);

  if cos_a <= 0. {
    return Black;
  }

  Color {
    r: ((obj.color.r as f64) * cos_a * (1. - obj.shininess) + (spot.color.r as f64) * cos_a * obj.shininess) as u8,
    g: ((obj.color.g as f64) * cos_a * (1. - obj.shininess) + (spot.color.g as f64) * cos_a * obj.shininess) as u8,
    b: ((obj.color.b as f64) * cos_a * (1. - obj.shininess) + (spot.color.b as f64) * cos_a * obj.shininess) as u8
  }
}

fn main() {
  let mut pixels = ~[Black, ..((WIDTH * HEIGHT) as uint)];

  let eye = ~Point { x: -300., y: 0., z: 200. };
  let spot = ~Spot {
    pos: Point { x: -300., y: 100., z: 200. },
    color: White
  };
  let sphere = ~Sphere {
    pos: Point { x: 0., y: 0., z: 0. },
    radius: 160.,
    shininess: 0.2,
    color: Red
  };

  for x in range(0., WIDTH) {
    for y in range(0., HEIGHT) {
      let vector = ~Point {
        x: 100.,
        y: (WIDTH / 2. - x),
        z: (HEIGHT / 2. - y)
      };

      // Always 0 :'(
      let closest = get_closest(sphere, eye, vector);

      // // Shadow
      // let inter = Point {
      //   x: eye.x + closest * vector.x,
      //   y: eye.y + closest * vector.y,
      //   z: eye.z + closest * vector.z
      // };
      // let light = Point {
      //   x: spot.pos.x - inter.x,
      //   y: spot.pos.y - inter.y,
      //   z: spot.pos.z - inter.z
      // };

      let inter = ~Point {
        x: (eye.x - sphere.pos.x) + closest * vector.x,
        y: (eye.y - sphere.pos.y) + closest * vector.y,
        z: (eye.z - sphere.pos.z) + closest * vector.z
      };
      let light = ~Point {
        x: spot.pos.x - inter.x,
        y: spot.pos.y - inter.y,
        z: spot.pos.z - inter.z
      };

      pixels[(y * WIDTH + x) as u32] = get_light(sphere, spot, inter, light);
    }
  }

  let img = png::Image {
    width: WIDTH as u32,
    height: HEIGHT as u32,
    color_type: png::RGB8,
    pixels: pixels.flat_map(|&Color { r: r, g: g, b: b }| { ~[r, g, b] })
  };

  png::store_png(&img, &Path::new("out.png"));
}
