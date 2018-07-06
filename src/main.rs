#![feature(duration_as_u128)]

extern crate ocl;
extern crate rand;

mod gpu;

fn main() -> ocl::Result<()> {
  gpu::run()
}
