use std::{
    iter::FromIterator,
    ops::{Add, AddAssign, Index},
};

#[derive(Copy, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct ModVec<T>(Vec<T>);

impl<T> ModVec<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Index<usize> for ModVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index % self.len())
    }
}

impl<T> FromIterator<T> for ModVec<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let inner = Vec::from_iter(iter);
        Self(inner)
    }
}

type World = Vec<ModVec<Tile>>;

impl Index<Pos> for World {
    type Output = Tile;

    fn index(&self, pos: Pos) -> &Self::Output {
        let line = self.index(pos.y as usize);
        line.index(pos.x as usize)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> World {
    input
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect()
}

fn traverse(world: &World, delta: &Pos) -> usize {
    let mut pos = Pos { x: 0, y: 0 };
    let mut count = 0;

    while pos.y < world.len() as u32 - delta.y {
        pos += delta;

        let tile = world[pos];
        if tile == Tile::Tree {
            count += 1;
        }
    }

    count
}

#[aoc(day3, part1)]
fn first(world: &World) -> usize {
    traverse(world, &Pos { x: 3, y: 1 })
}

#[aoc(day3, part2)]
fn second(world: &World) -> usize {
    let deltas = [
        Pos { x: 1, y: 1 },
        Pos { x: 3, y: 1 },
        Pos { x: 5, y: 1 },
        Pos { x: 7, y: 1 },
        Pos { x: 1, y: 2 },
    ];

    deltas.iter().map(|d| traverse(&world, d)).product()
}
