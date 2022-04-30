extern crate crossbeam;

use std::ptr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

extern crate rand;
use self::rand::Rng;

mod world;
use self::world::{Sector, World};

pub fn run() {
  let mut world = create_world();

  // let n = 61;

  // {
  //   macro_rules! cell {
  //     ($x:expr, $y:expr) => {
  //       world.set($x + n, $y, true);
  //     };
  //   }

  //   cell!(25, 1);

  //   cell!(23, 2);
  //   cell!(25, 2);

  //   cell!(13, 3);
  //   cell!(14, 3);
  //   cell!(21, 3);
  //   cell!(22, 3);
  //   cell!(35, 3);
  //   cell!(36, 3);

  //   cell!(12, 4);
  //   cell!(16, 4);
  //   cell!(21, 4);
  //   cell!(22, 4);
  //   cell!(35, 4);
  //   cell!(36, 4);

  //   cell!(1, 5);
  //   cell!(2, 5);
  //   cell!(11, 5);
  //   cell!(17, 5);
  //   cell!(21, 5);
  //   cell!(22, 5);

  //   cell!(1, 6);
  //   cell!(2, 6);
  //   cell!(11, 6);
  //   cell!(15, 6);
  //   cell!(17, 6);
  //   cell!(18, 6);
  //   cell!(23, 6);
  //   cell!(25, 6);

  //   cell!(11, 7);
  //   cell!(17, 7);
  //   cell!(25, 7);

  //   cell!(12, 8);
  //   cell!(16, 8);

  //   cell!(13, 9);
  //   cell!(14, 9);
  // }

  // {
  //   macro_rules! cell {
  //     ($x:expr, $y:expr) => {
  //       world.set($x + n, 49 - $y, true);
  //     };
  //   }

  //   cell!(25, 1);

  //   cell!(23, 2);
  //   cell!(25, 2);

  //   cell!(13, 3);
  //   cell!(14, 3);
  //   cell!(21, 3);
  //   cell!(22, 3);
  //   cell!(35, 3);
  //   cell!(36, 3);

  //   cell!(12, 4);
  //   cell!(16, 4);
  //   cell!(21, 4);
  //   cell!(22, 4);
  //   cell!(35, 4);
  //   cell!(36, 4);

  //   cell!(1, 5);
  //   cell!(2, 5);
  //   cell!(11, 5);
  //   cell!(17, 5);
  //   cell!(21, 5);
  //   cell!(22, 5);

  //   cell!(1, 6);
  //   cell!(2, 6);
  //   cell!(11, 6);
  //   cell!(15, 6);
  //   cell!(17, 6);
  //   cell!(18, 6);
  //   cell!(23, 6);
  //   cell!(25, 6);

  //   cell!(11, 7);
  //   cell!(17, 7);
  //   cell!(25, 7);

  //   cell!(12, 8);
  //   cell!(16, 8);

  //   cell!(13, 9);
  //   cell!(14, 9);
  // }

  // {
  //   macro_rules! cell {
  //     ($x:expr, $y:expr) => {
  //       world.set(199 - $x - n, $y, true);
  //     };
  //   }

  //   cell!(25, 1);

  //   cell!(23, 2);
  //   cell!(25, 2);

  //   cell!(13, 3);
  //   cell!(14, 3);
  //   cell!(21, 3);
  //   cell!(22, 3);
  //   cell!(35, 3);
  //   cell!(36, 3);

  //   cell!(12, 4);
  //   cell!(16, 4);
  //   cell!(21, 4);
  //   cell!(22, 4);
  //   cell!(35, 4);
  //   cell!(36, 4);

  //   cell!(1, 5);
  //   cell!(2, 5);
  //   cell!(11, 5);
  //   cell!(17, 5);
  //   cell!(21, 5);
  //   cell!(22, 5);

  //   cell!(1, 6);
  //   cell!(2, 6);
  //   cell!(11, 6);
  //   cell!(15, 6);
  //   cell!(17, 6);
  //   cell!(18, 6);
  //   cell!(23, 6);
  //   cell!(25, 6);

  //   cell!(11, 7);
  //   cell!(17, 7);
  //   cell!(25, 7);

  //   cell!(12, 8);
  //   cell!(16, 8);

  //   cell!(13, 9);
  //   cell!(14, 9);
  // }

  // {
  //   macro_rules! cell {
  //     ($x:expr, $y:expr) => {
  //       world.set(199 - $x - n, 49 - $y, true);
  //     };
  //   }

  //   cell!(25, 1);

  //   cell!(23, 2);
  //   cell!(25, 2);

  //   cell!(13, 3);
  //   cell!(14, 3);
  //   cell!(21, 3);
  //   cell!(22, 3);
  //   cell!(35, 3);
  //   cell!(36, 3);

  //   cell!(12, 4);
  //   cell!(16, 4);
  //   cell!(21, 4);
  //   cell!(22, 4);
  //   cell!(35, 4);
  //   cell!(36, 4);

  //   cell!(1, 5);
  //   cell!(2, 5);
  //   cell!(11, 5);
  //   cell!(17, 5);
  //   cell!(21, 5);
  //   cell!(22, 5);

  //   cell!(1, 6);
  //   cell!(2, 6);
  //   cell!(11, 6);
  //   cell!(15, 6);
  //   cell!(17, 6);
  //   cell!(18, 6);
  //   cell!(23, 6);
  //   cell!(25, 6);

  //   cell!(11, 7);
  //   cell!(17, 7);
  //   cell!(25, 7);

  //   cell!(12, 8);
  //   cell!(16, 8);

  //   cell!(13, 9);
  //   cell!(14, 9);
  // }

  let w = world.width;
  let h = world.height;

  #[cfg_attr(rustfmt, rustfmt_skip)]
  let sectors = vec![
    Sector::new(0,     0,     w / 2, h / 2),
    Sector::new(w / 2, 0,     w / 2, h / 2),
    Sector::new(0,     h / 2, w / 2, h / 2),
    Sector::new(w / 2, h / 2, w / 2, h / 2),
  ];

  // let mut world = Arc::new(world);

  let mut generation = 0;
  // let generation = Arc::new(Mutex::new(0));

  // {
  //   let world_ptr = &mut world as *mut World;
  //   let world_copy = unsafe { &mut *world_ptr };

  //   let thread_generation = generation.clone();

  //   thread::spawn(move || loop {
  //     print!("{}", world_copy);
  //     println!("{}", thread_generation.lock().unwrap());
  //     thread::sleep(Duration::from_millis(10));
  //     print!("{}[{}A", 27 as char, world_copy.height + 1);
  //   });
  // }

  // loop {
  //   measure_time(format!("generation #{}", generation).as_str(), || {
  //     // world = parallel_next_generation(&world, &sectors);
  //     parallel_next_generation_2(&mut world, &sectors);
  //   });
  //   generation += 1;
  // }

  let mut next_world = World::new(world.width, world.height);

  crossbeam::scope(|scope| {
    let world_ptr = &mut world as *mut World;
    let next_world_ptr = &mut next_world as *mut World;

    let mut sector_senders = Vec::with_capacity(4);
    let mut done_receivers = Vec::with_capacity(4);

    for _ in 0..4 {
      let thread_world = unsafe { &mut *world_ptr };
      let thread_next_world = unsafe { &mut *next_world_ptr };

      let (sector_sender, sector_receiver) = mpsc::channel();
      let (done_sender, done_receiver) = mpsc::channel();

      scope.spawn(move || loop {
        let sector = sector_receiver.recv().unwrap();

        let sector_world = thread_world.next_generation(sector);

        for y in 0..sector.height {
          for x in 0..sector.width {
            let cell = sector_world.get(x, y);
            thread_next_world.set(sector.x + x, sector.y + y, cell);
          }
        }

        done_sender.send(()).unwrap();
      });

      sector_senders.push(sector_sender);
      done_receivers.push(done_receiver);
    }

    loop {
      // println!("generation {}", generation);
      // print!("{}", world);
      // print!("{}[{}A", 27 as char, world.height + 1);

      measure_time(format!("generation #{}", generation).as_str(), || {
        for index in 0..4 {
          let tx = &sector_senders[index];
          let sector = &sectors[index];
          tx.send(sector).unwrap();
        }

        for index in 0..4 {
          let rx = &done_receivers[index];
          rx.recv().unwrap();
        }

        unsafe {
          ptr::swap(world_ptr, next_world_ptr);
        }
      });

      generation += 1;
    }
  });
}

fn parallel_next_generation_unsafe(
  world: &World,
  sectors: &Vec<Sector>,
) -> World {
  let mut next_world = World::new(world.width, world.height);

  crossbeam::scope(|scope| {
    let next_world_ptr = &mut next_world as *mut World;

    for sector in sectors {
      let thread_next_world = unsafe { &mut *next_world_ptr };

      scope.spawn(move || {
        let sector_world = world.next_generation(sector);

        for y in 0..sector.height {
          for x in 0..sector.width {
            let cell = sector_world.get(x, y);
            thread_next_world.set(sector.x + x, sector.y + y, cell);
          }
        }
      });
    }
  });

  next_world
}

fn parallel_next_generation_safe(
  world: &mut Arc<World>,
  sectors: &Vec<Sector>,
) {
  let threads = measure_time("threads", || {
    crossbeam::scope(|scope| {
      sectors
        .iter()
        .map(|sector| {
          let thread_world = world.clone();
          scope.spawn(move || (sector, thread_world.next_generation(sector)))
        })
        .collect::<Vec<_>>()
    })
  });

  measure_time("merge", || {
    for thread in threads {
      let (sector, sector_world) = thread.join();
      for y in 0..sector.height {
        for x in 0..sector.width {
          let cell = sector_world.get(x, y);
          Arc::get_mut(world)
            .unwrap()
            .set(sector.x + x, sector.y + y, cell);
        }
      }
    }
  });
}

fn create_world() -> World {
  measure_time("create world", || {
    let mut world = World::new(10_000, 10_000);
    // let mut world = World::new(200, 50);

    let mut rng = rand::thread_rng();
    for _ in 0..100_000_000 as u32 {
      // for _ in 0..5000 as u32 {
      let x = rng.gen_range(0, world.width);
      let y = rng.gen_range(0, world.height);
      world.set(x, y, true);
    }

    world
  })
}

pub fn measure_time<F, R>(name: &str, f: F) -> R
where
  F: FnOnce() -> R,
{
  let start_time = Instant::now();
  let ret = f();
  println!("{} - {} µs", name, start_time.elapsed().as_micros());
  ret
}
