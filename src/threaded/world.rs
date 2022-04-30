use std::fmt;

// extern crate bit_vec;
// use self::bit_vec::BitVec;

pub struct World {
  pub width: usize,
  pub height: usize,
  // data: BitVec,
  data: Vec<bool>,
}

impl World {
  pub fn new(width: usize, height: usize) -> Self {
    World {
      width,
      height,
      // data: BitVec::from_elem(width * height, false),
      data: vec![false; width * height],
    }
  }

  pub fn get(&self, x: usize, y: usize) -> bool {
    self.assert_in_bounds(x, y);
    // self.data.get(y * self.width + x).unwrap()
    self.data[y * self.width + x]
  }

  pub fn set(&mut self, x: usize, y: usize, cell: bool) {
    self.assert_in_bounds(x, y);
    // self.data.set(y * self.width + x, cell);
    self.data[y * self.width + x] = cell;
  }

  fn assert_in_bounds(&self, x: usize, y: usize) {
    debug_assert!(
      x < self.width,
      "x out of bounds: the width is {} but the x is {}",
      self.width,
      x
    );

    debug_assert!(
      y < self.height,
      "y out of bounds: the height is {} but the y is {}",
      self.height,
      y
    );
  }

  pub fn next_generation(&self, sector: &Sector) -> Self {
    let mut next_world = World::new(sector.width, sector.height);

    for y in 0..sector.height {
      for x in 0..sector.width {
        let cell = self.get(sector.x + x, sector.y + y);

        let n = self.count_neighbors(sector.x + x, sector.y + y);
        let next_cell = if cell { n >= 2 && n <= 3 } else { n == 3 };

        next_world.set(x, y, next_cell);
      }
    }

    next_world
  }

  fn count_neighbors(&self, x: usize, y: usize) -> u8 {
    let mut result = 0;

    macro_rules! neighbor {
      ($condition:expr, $x:expr, $y:expr) => {
        if $condition && self.get($x, $y) {
          result += 1;
        }
      };
    }

    let w = self.width;
    let h = self.height;

    #[cfg_attr(rustfmt, rustfmt_skip)] {
      neighbor!(y > 0                 , x,     y - 1); // top
      neighbor!(x < w - 1 && y > 0    , x + 1, y - 1); // top right
      neighbor!(x < w - 1             , x + 1, y    ); // right
      neighbor!(x < w - 1 && y < h - 1, x + 1, y + 1); // bottom right
      neighbor!(y < h - 1             , x,     y + 1); // bottom
      neighbor!(x > 0     && y < h - 1, x - 1, y + 1); // bottom left
      neighbor!(x > 0                 , x - 1, y    ); // left
      neighbor!(x > 0     && y > 0    , x - 1, y - 1); // top left
    }

    result
  }
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        write!(f, "{}", if self.get(x, y) { 'x' } else { '.' })?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

#[derive(Debug)]
pub struct Sector {
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
}

impl Sector {
  pub fn new(x: usize, y: usize, width: usize, height: usize) -> Sector {
    Sector {
      x,
      y,
      width,
      height,
    }
  }
}
