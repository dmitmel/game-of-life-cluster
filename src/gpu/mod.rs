extern crate ocl;
use self::ocl::{
  Buffer, Context, Device, Kernel, Platform, Program, Queue,
  Result as OclResult, SpatialDims,
};

extern crate rand;
use self::rand::Rng;

use std::time::Instant;

type Cell = u8;
type World = Buffer<Cell>;

const PROGRAM_SOURCE: &str = include_str!("program.cl");
const KERNEL_NAME: &str = "next_generation";

pub fn run() -> OclResult<()> {
  let width = 200;
  let height = 50;
  let dimensions = SpatialDims::Two(width, height);

  let platform = time("get_platform", get_platform);
  let device = time("get_device", || get_device(&platform))?;
  let context = time("create_context", || create_context(platform, &device))?;

  let program = time("compile_program", || compile_program(&context, &device))?;

  let queue = time("create_queue", || create_queue(&context, device))?;
  let world_a = time("create_world", || create_world(&queue, &dimensions))?;
  let world_b = time("create_world", || create_world(&queue, &dimensions))?;

  let kernel = time("create_kernel", || {
    create_kernel(&program, &queue, &dimensions)
  })?;

  let mut tmp_data = vec![0; dimensions.to_len()];
  time("fill_world", || {
    generate_world_data(&dimensions, &mut tmp_data);
    world_a.write(&tmp_data).enq()
  })?;

  println!();

  for n in 0.. {
    let world = if n % 2 == 0 { &world_a } else { &world_b };
    let next_world = if n % 2 == 0 { &world_b } else { &world_a };

    time(format!("generation #{}", n).as_str(), || {
      next_generation(&kernel, world, next_world)
    })?;

    next_world.read(&mut tmp_data).enq()?;
    print_world_data(&tmp_data, &dimensions);
    move_cursor_up(height as u16 + 1);
  }

  Ok(())
}

pub fn time<F, R>(name: &str, f: F) -> R
where
  F: FnOnce() -> R,
{
  let start_time = Instant::now();
  let ret = f();
  println!("{} - {} µs", name, start_time.elapsed().as_micros());
  ret
}

fn get_platform() -> Platform {
  Platform::default()
}

fn get_device(platform: &Platform) -> OclResult<Device> {
  Device::first(platform)
}

fn create_context(platform: Platform, device: &Device) -> OclResult<Context> {
  Context::builder()
    .platform(platform)
    .devices(device)
    .build()
}

fn compile_program(context: &Context, device: &Device) -> OclResult<Program> {
  Program::builder()
    .devices(device)
    .src(PROGRAM_SOURCE)
    .build(&context)
}

fn create_queue(context: &Context, device: Device) -> OclResult<Queue> {
  Queue::new(context, device, None)
}

fn create_world(queue: &Queue, dimensions: &SpatialDims) -> OclResult<World> {
  World::builder()
    .queue(queue.clone())
    .flags(ocl::flags::MEM_READ_WRITE)
    .len(dimensions)
    .fill_val(0)
    .build()
}

fn create_kernel(
  program: &Program,
  queue: &Queue,
  dimensions: &SpatialDims,
) -> OclResult<Kernel> {
  Kernel::builder()
    .program(program)
    .name(KERNEL_NAME)
    .queue(queue.clone())
    .global_work_size(dimensions)
    .arg(None::<&World>)
    .arg(None::<&World>)
    .build()
}

fn generate_world_data(dimensions: &SpatialDims, dest: &mut Vec<Cell>) {
  let width = dimensions[0];
  let height = dimensions[1];

  let mut rng = rand::thread_rng();
  for y in 0..height {
    for x in 0..width {
      dest[x + y * width] = rng.gen_bool(0.5) as Cell;
    }
  }
}

fn next_generation(
  kernel: &Kernel,
  world: &World,
  next_world: &World,
) -> OclResult<()> {
  kernel.set_arg(0, world)?;
  kernel.set_arg(1, next_world)?;
  unsafe { kernel.enq() }
}

fn print_world_data(data: &[Cell], dimensions: &SpatialDims) {
  let width = dimensions[0];
  let height = dimensions[1];

  for y in 0..height {
    for x in 0..width {
      print!("{}", if data[x + y * width] > 0 { 'x' } else { ' ' });
    }
    println!();
  }
}

fn move_cursor_up(lines: u16) {
  print!("\x1b[{}A", lines);
}
