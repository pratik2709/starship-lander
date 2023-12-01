extern crate rand;
extern crate ndarray;

use std::collections::VecDeque;
use ndarray::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::collections::HashSet;


pub struct Landing {
    roughness: i32,
    start: i32,
    end: i32,
}

impl Landing {
    pub fn new(roughness: i32) -> Self {
        Landing {
            roughness,
            start: -800,
            end: 800,
        }
    }

    fn render_heights(&self, heights: &mut Vec<f32>) -> Vec<f32> {
        self.normalize(heights, 50.0, 100.0)
    }

    fn normalize(&self,heights: &Vec<f32>, new_lower_bound: f32, new_upper_bound: f32) -> Vec<f32> {
        let min = heights.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = heights.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;
        let new_range = new_upper_bound - new_lower_bound;
        heights.iter().map(|&a| (a - min) * new_range / range + new_lower_bound).collect()
    }

    fn place_multipliers(&mut self, points: &mut Vec<DVec2>, num_multipliers: usize, min_index_distance: usize,
                         max_multiplier: f64) {
        let len = points.len();
        let mut rng = rand::thread_rng();

        // The multipliers are placed at random indexes within the points array.
        for _ in 0..num_multipliers {
            // The multiplier index should be a safe distance from both ends of the points array.
            let index = rng.gen_range(min_index_distance..len - min_index_distance);

            // Avoid placing multipliers near the terrain edges to ensure pleasant looking landing zones
            if index < min_index_distance || index > len - min_index_distance {
                continue;
            }

            // Generate a random multiplier
            let multiplier = rng.gen_range(1.0..max_multiplier);

            // Apply the multiplier
            points[index].y *= multiplier;
            // for j in 0..length {
            //     points[index].y = multiplier;
            // }
        }
    }

    fn setup_data(&mut self) -> Vec<DVec2> {
        let mut points: Vec<DVec2> = Vec::new();
        for i in 0..2000 {
            let x = i as f64 * 8.0;
            let perlin = Perlin::new().set_seed(60);
            let mut y: f64 = perlin.get([x as f64 / 500.0, 100.0]) * 156.0;
            y += perlin.get([x as f64 / 200.0, 100.0]) * 80.0;
            y += perlin.get([x as f64 / 40.0, 100.0]) * 20.0;
            y = y.min(150.0) + 200.0;

            points.push(DVec2::new(x as f64, y as f64));

            points[i].x *= 0.8;
            points[i].y *= 0.8;
            points[i].y -= 80.0;
        }

        // Detect valleys and flatten them
        for i in 1..(points.len()-1) {
            let prev_y = points[i - 1].y;
            let curr_y = points[i].y;
            let next_y = points[i + 1].y;

            // If current y is a valley
            if curr_y < prev_y && curr_y < next_y {
                points[i].y = curr_y;  // Flatten this point to its current elevation

                // Optionally flatten the surrounding points to the same elevation
                points[i - 2].y = curr_y;
                points[i - 1].y = curr_y;
                points[i + 1].y = curr_y;
                points[i + 2].y = curr_y;
            }
        }

        // interpolate points
        let mut points1 = Vec::new();
        let steps = 10;
        for i in 0..(points.len()-1) {
            let p1 = points[i];
            let p2 = points[i+1];

            for s in 0..steps {
                let t = s as f64 / steps as f64;
                let x = p1.x * (1.0 - t) + p2.x * t;
                let y = p1.y * (1.0 - t) + p2.y * t;

                points1.push(DVec2::new(x, y));
            }
        }

        points1
    }


    pub fn generate_exact_terrain(&mut self, seed: u64, height: i32,
                                  width: i32, start: f32, end: f32, r: bool) -> Vec<(f32)> {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);
        let mut heights: Vec<f32> = vec![0.0; width as usize];
        heights[0] = start;
        heights[width as usize - 1] = end;
        let mut queue: VecDeque<(f32, f32, f32)> = VecDeque::new();
        queue.push_back((0.0, (width - 1) as f32, self.roughness as f32));

        while !queue.is_empty() {
            let (left, right, randomness) = queue.pop_front().unwrap();
            let mut center = (left + right + 1.0) / 2.0;
            center = center.floor();
            heights[center as usize] = (heights[left as usize] + heights[right as usize]) / 2.0;
            heights[center as usize] = heights[center as usize] + rng.gen_range(-randomness..randomness);
            // println!("{:?}", randomness);
            if right - left > 2.0 {
                queue.push_back((left, center, (randomness / 2.0) as f32));
                queue.push_back((center, right, (randomness / 2.0) as f32));
            }
        }

        // for i in 0..(heights.len() - 5) {
        //     if rng.gen_range(0..50) < 1 { // 10% chance to generate a platform at each position
        //         let platform_height = heights[i];
        //         for j in 0..30 {
        //             heights[i + j] = platform_height;
        //         }
        //     }
        // }
        let reg = self.select_contiguous_sub_array_regions(&heights);
        heights = self.create_platforms(reg.clone(),heights.clone());
        // println!("{:?}", reg);
        // println!("{:?}", heights.clone());

        // if r {
        //     self.render_heights(&mut heights)
        // } else {
            // println!("{:?}", heights);

        heights
        // }
    }

    fn select_contiguous_sub_array_regions(&self, old_heightmap: &Vec<f32>) -> Vec<(usize, usize)> {
        let heightmap = old_heightmap[..700].to_vec();
        let mut rng = rand::thread_rng();
        let mut regions = vec![];
        let mut seen: Vec<f32> = Vec::new();
        let ar = 0.02;
        let ln: usize = heightmap.len();
        let mut sample_size: usize = 0;

        while sample_size < (ln as f32 * 0.10) as usize {
            let size = (ln as f32 * ar) as usize;
            if let ch = rng.gen_range(0..heightmap.len() - size + 1) {
                let rn = &heightmap[ch..ch+size];
                if rn.len() == size && !rn.iter().any(|x| seen.contains(x)) {
                    regions.push((ch, ch + size));
                    seen.extend(rn);
                    sample_size += rn.len();
                }
            }
        }
        // regions.push((600, 1000));
        // println!("{}", (ln as f32 * ar) as usize);
        regions
    }

    fn create_platforms(&self, points_to_flatten: Vec<(usize, usize)>, mut height_map: Vec<f32>) -> Vec<f32> {
        for point in points_to_flatten {
            let height_value = height_map[point.0];
            // if (height_map[point.1] - height_value).abs() > 5.0 {
            //     continue;
            // }
            for i in point.0..=point.1 {
                height_map[i] = height_value;
            }
        }
        height_map
    }

    fn convert_to_vec2(&self, points: Vec<(i32, i32)>) -> Vec<Vec2> {
        points.into_iter().map(|(x, y)| Vec2::new(x as f32, y as f32)).collect()
    }

    fn format_heights(&self, points: Vec<i32>, width: i32, height: i32) -> Vec<Vec2> {
        let mut new_points: Vec<Vec2> = self.format_points(points);
        new_points.push(Vec2::new(0.0, height as f32));
        new_points.push(Vec2::new(width as f32, height as f32));
        new_points
    }

    fn format_points(&self, points: Vec<i32>) -> Vec<Vec2> {
        let org_len = points.len();
        (1..org_len).map(|i| Vec2::new(i as f32, points[i] as f32)).collect()
    }

}
