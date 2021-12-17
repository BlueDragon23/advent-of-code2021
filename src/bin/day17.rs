use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;
use std::ops::RangeInclusive;

use itertools::Itertools;

const PART: u32 = 1;

// target area: x=155..182, y=-117..-67
const MAX_STEPS: i32 = 100_000;

fn main() {
  let target_x = 155..=182;
  let target_y = -117..=-67;


  // v_x must be at least n * (n+1) / 2 == 155, n = 18
  // v_x must be at most 27 by the time it reaches the target, else guaranteed to miss
  let mut successful;
  let mut success_count = 0;
  for v_x in 18..1000 {
    for v_y in -118..1000 {
      successful = check_trajectory(v_x, v_y, &target_x, &target_y);
      if successful {
        success_count += 1;
      }
    }
  }
  println!("Success count was {}", success_count);
}

// Determine whether a given initial velocity reaches the target range
fn check_trajectory(init_v_x: i32, init_v_y: i32, target_x: &RangeInclusive<i32>, target_y: &RangeInclusive<i32>) -> bool {
  let mut x = 0;
  let mut y = 0;
  let mut v_x = init_v_x;
  let mut v_y = init_v_y;
  let mut max_y = 0;
  for _ in 0..MAX_STEPS {
    x += v_x;
    y += v_y;
    if y > max_y { 
      max_y = y;
    }
    // println!("x: {}, y: {}", x, y);
    v_x = if v_x > 0 { v_x - 1 } else if v_x < 0 { v_x + 1 } else { 0 };
    v_y -= 1;
    if x > *target_x.end() || x < *target_x.start() && v_x == 0 || y < *target_y.start() && v_y < 0 {
      // println!("{}, {} missed", init_v_x, init_v_y);
      return false;
    }
    if target_x.contains(&x) && target_y.contains(&y) {
      println!("Max y was {} from {}, {}", max_y, init_v_x, init_v_y);
      return true;
    }
  }
  println!("{}, {} ran out of steps", init_v_x, init_v_y);
  false
}