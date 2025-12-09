use std::{collections::BTreeMap, fmt::Display};

pub mod p1;
pub mod p2;

#[derive(Debug)]
pub struct Point3d {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Point3d {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Point3d { x, y, z }
    }

    pub fn distance(&self, other: &Point3d) -> f64 {
        let num = ((self.x as isize - other.x as isize).pow(2)
            + (self.y as isize - other.y as isize).pow(2)
            + (self.z as isize - other.z as isize).pow(2)) as usize;

        //num.sqrt()
        f64::sqrt(num as f64)
    }

    pub fn parse(input: &str) -> Self {
        let mut split = input.split(",");

        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }
}

pub struct UnionFind {
    parent: BTreeMap<usize, usize>,
    size: BTreeMap<usize, usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent: BTreeMap::new(),
            size: BTreeMap::new(),
        }
    }

    pub fn parent(&self, x: &usize) -> usize {
        *self.parent.get(x).unwrap_or(&x)
    }

    pub fn size(&self, x: &usize) -> usize {
        *self.size.get(x).unwrap_or(&1)
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent(&x) != x {
            self.parent.insert(x, self.parent(&self.parent(&x)));
            x = self.parent(&x);
        }
        return x;
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) {
        // Replace nodes by roots
        x = self.find(x);
        y = self.find(y);

        if x == y {
            return; // x and y are already in the same set
        }

        // If necessary, swap variables to ensure that
        // x has at least as many descendants as y
        if self.size(&x) < self.size(&y) {
            self.parent.insert(y, x);
            self.size.insert(x, self.size(&x) + self.size(&y));
        } else {
            self.parent.insert(x, y);
            self.size.insert(y, self.size(&y) + self.size(&x));
        }
    }

    pub fn sizes(&self) -> Vec<usize> {
        self.size
            .iter()
            .filter(|(n, size)| self.parent(n) == **n)
            .map(|(_, size)| *size)
            .collect()
    }
}
