use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

aoc_day!(2017, 20);

fn answer_one() -> String {
    let mut particles = parse(&input());

    simulate(&mut particles, 1000, false);

    particles
        .iter()
        .min_by_key(|(_, p)| p.position.distance(&Vector3::ZERO))
        .unwrap()
        .0
        .to_string()
}

fn answer_two() -> String {
    let mut particles = parse(&input());

    simulate(&mut particles, 1000, true);

    particles.len().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector3 {
    const ZERO: Vector3 = Vector3::new(0, 0, 0);

    const fn new(x: isize, y: isize, z: isize) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn distance(&self, other: &Vector3) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl FromStr for Particle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_regex =
        Regex::new(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$").unwrap();
        let captures = line_regex.captures(s).unwrap();

        let position = Vector3::new(
            captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let velocity = Vector3::new(
            captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(5).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(6).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let acceleration = Vector3::new(
            captures.get(7).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(8).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(9).unwrap().as_str().parse::<isize>().unwrap(),
        );
        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

fn parse(input: &str) -> HashMap<usize, Particle> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| (i, l.parse().unwrap()))
        .collect()
}

fn simulate(particles: &mut HashMap<usize, Particle>, steps: usize, collisions: bool) {
    for _ in 0..steps {
        for particle in particles.values_mut() {
            particle.velocity = particle.velocity + particle.acceleration;
            particle.position = particle.position + particle.velocity;
        }

        if collisions {
            let to_remove: Vec<_> = particles
                .iter()
                .filter(|(_, p1)| {
                    particles
                        .values()
                        .filter(|p2| p1.position == p2.position)
                        .count()
                        > 1
                })
                .map(|(i, _)| *i)
                .collect();
            for p in to_remove {
                particles.remove(&p);
            }
        }
    }
}
