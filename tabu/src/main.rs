//! Tabu Search in Rust 

extern crate rand;

use std::ops::{Index, IndexMut};
use std::mem;
use rand::distributions::{IndependentSample, Range};

#[derive(Copy,Clone,Debug,PartialEq)]
struct Coordinate(i32, i32);
type City = Coordinate;
type Cities = Vec<Coordinate>;

#[derive(Clone,Debug,PartialEq)]
struct Edge(usize, usize);
type EdgePair = Vec<Edge>;
type Permutation = Vec<usize>;
type TabuList = Vec<Edge>;

#[derive(Clone,Debug)]
struct Candidate {
    vector: Permutation,
    edges: EdgePair,
    cost: i32
}

fn euclidean_2d(c1: Coordinate, c2: Coordinate) -> i32 {
    let thing = ((c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2)) as f32;
    thing.sqrt().round() as i32
}

fn cost(cities: &Cities, permutation: Permutation) -> i32 {
    let mut distance: i32 = 0;
    let mut c2: usize;
    for (i, c1) in permutation.iter().enumerate() {
        if i == cities.len() - 1 {
            c2 = permutation[0];
        } else {
            c2 = permutation[i + 1];
        }

        distance += euclidean_2d(cities[*c1], cities[c2])
    }

    distance
}

fn random_permutation(cities: &Cities) -> Permutation {
    let mut rng = rand::thread_rng();
    let len = cities.len();
    let mut permutation: Permutation = Vec::with_capacity(len);

    // Initialize our permutation
    for i in 0..len as usize {
        permutation.push(i);
    }

    let between = Range::new(0, len);
    let mut r: usize;

    for i in 0..len {
        r = between.ind_sample(&mut rng);
        permutation.swap(i, r);
    }

    permutation
}

fn stochastic_edge_generation(parent: Permutation) -> (Permutation, EdgePair) {
    let mut permutation = parent;
    let mut rng = rand::thread_rng();
    let len = permutation.len();
    let between = Range::new(1, len);

    let (mut c1, mut c2) = (between.ind_sample(&mut rng), between.ind_sample(&mut rng));

    let exclude = match c1 {
        0              => [c1, c1 + 1, len - 1],
        x if c1 == len => [c1, c1 - 1, 0],
        _              => [c1, c1 + 1, c1 - 1],
    };

    while exclude.contains(&c2) {
        c2 = between.ind_sample(&mut rng);
    }

    if c2 < c1 {
        mem::swap(&mut c1, &mut c2);
    }

    let iter = (c1..c2).into_iter().zip((c2..c1));
    for (x,y) in iter {
        println!("x: {}, y: {}", x, y);
        permutation.swap(x,y);
    }

    // println!("c1: {} ; c2: {}; len: {}", c1, c2, len);

    let mut xn = c1;
    if c1 == 0 {
        xn = len - 1;
    }

    let edgepair: EdgePair = vec![
        Edge (permutation[xn], permutation[c1]),
        Edge (permutation[c2-1], permutation[c2])
    ];

    (permutation, edgepair)
}

fn is_tabu(permutation: &Permutation, tabu_list: &TabuList) -> bool {
    let mut c2: usize;
    for (i, c1) in permutation.iter().enumerate() {
        if i == permutation.len() - 1 {
            c2 = permutation[0];
        } else {
            c2 = permutation[i+1];
        }
        for forbidden in tabu_list {
            if forbidden.0 == *c1 && forbidden.1 == c2 {
                return true;
            }
        }
    }
    false
}

fn generate_candidate(best: Candidate, tabu_list: &TabuList, cities: &Cities) -> Candidate {
    let (perm, edges) = stochastic_edge_generation(best.vector);
    //while is_tabu(&perm, tabu_list) {
    //    let (perm, edges) = stochastic_edge_generation(vector);
    //}

    Candidate {
        vector: perm.clone(),
        cost: cost(&cities, perm),
        edges: edges,
    }
}

fn search(cities: &Cities, tabu_size: usize, candidate_size: usize, max_iter: usize) -> Candidate {
    let perm = random_permutation(cities);
    let mut current = Candidate {
        vector: perm.clone(),
        cost: cost(&cities, perm.clone()),
        edges: vec![Edge(0,0),Edge(0,0)],
    };

    let mut best: Candidate = current.clone();
    let mut best_candidate: Candidate;
    let tabu_list: TabuList = vec![];

    let mut candidates: Vec<Candidate> = vec![generate_candidate(current.clone(), &tabu_list, cities); candidate_size];
    for i in 0..max_iter {
        for (x,i) in candidates.clone().iter().enumerate() {
            candidates[x] = generate_candidate(current.clone(), &tabu_list, cities);
            // println!("{:?}", candidates[x]);
        }
        candidates.sort_by(|a,b| a.cost.cmp(&b.cost) );
        best_candidate = candidates[0].clone();
        if best_candidate.cost < current.cost {
            current = best_candidate.clone();
            if best_candidate.cost < best.cost {
                best = best_candidate;
                println!("best: {:?}", best);
            }
        }
    }

    return best
}


fn main() {
    let test: Cities = vec![
        Coordinate (1,1),
        Coordinate (689,291),
        Coordinate (801,724),
        Coordinate (388,143),
        Coordinate (143,832),
        Coordinate (485,484),
        Coordinate (627,231),
        Coordinate (610,311),
        Coordinate (549,990),
        Coordinate (220,28),
        Coordinate (66,496),
        Coordinate (693,988),
        Coordinate (597,372),
        Coordinate (753,222),
        Coordinate (885,639),
        Coordinate (897,594),
        Coordinate (482,635),
        Coordinate (379,490),
        Coordinate (923,781),
        Coordinate (352,867),
        Coordinate (834,713),
        Coordinate (133,344),
        Coordinate (835,949),
        Coordinate (667,695),
        Coordinate (956,850),
        Coordinate (535,170),
        Coordinate (583,406)
    ];

    let berlin52: Cities =
        vec!(
            Coordinate (565,575),
            Coordinate (25,185),
            Coordinate (345,750),
            Coordinate (945,685),
            Coordinate (845,655),
            Coordinate (880,660),
            Coordinate (25,230),
            Coordinate (525,1000),
            Coordinate (580,1175),
            Coordinate (650,1130),
            Coordinate (1605,620),
            Coordinate (1220,580),
            Coordinate (1465,200),
            Coordinate (1530,5),
            Coordinate (845,680),
            Coordinate (725,370),
            Coordinate (145,665),
            Coordinate (415,635),
            Coordinate (510,875),
            Coordinate (560,365),
            Coordinate (300,465),
            Coordinate (520,585),
            Coordinate (480,415),
            Coordinate (835,625),
            Coordinate (975,580),
            Coordinate (1215,245),
            Coordinate (1320,315),
            Coordinate (1250,400),
            Coordinate (660,180),
            Coordinate (410,250),
            Coordinate (420,555),
            Coordinate (575,665),
            Coordinate (1150,1160),
            Coordinate (700,580),
            Coordinate (685,595),
            Coordinate (685,610),
            Coordinate (770,610),
            Coordinate (795,645),
            Coordinate (720,635),
            Coordinate (760,650),
            Coordinate (475,960),
            Coordinate (95,260),
            Coordinate (875,920),
            Coordinate (700,500),
            Coordinate (555,815),
            Coordinate (830,485),
            Coordinate (1170,65),
            Coordinate (830,610),
            Coordinate (605,625),
            Coordinate (595,360),
            Coordinate (1340,725),
            Coordinate (1740,245)
        );


    let max_iter = 500;
    let tabu_list_size = 15;
    let max_candidates = 50;
    let best = search(&berlin52, tabu_list_size, max_candidates, max_iter);

    println!("{:?}", best);
    println!("Cost: {}", best.cost);
}
