#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_macros)]
macro_rules! read_cols {
    ($($t:ty),+) => {{
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();
        
        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[derive(Clone)]
struct Record {
    parents: Vec<usize>,
    cost: usize,
}

impl Record {
    fn new() -> Record {
        Record { parents: Vec::new(), cost: usize::max_value() }
    }

    fn to_handle(&self, vertex: usize) -> Handle {
        Handle { vertex : vertex, cost: self.cost }
    }
}

#[derive(PartialEq, Eq)]
struct Handle {
    vertex: usize,
    cost: usize,
}

impl Ord for Handle {
    fn cmp(&self, other: &Handle) -> Ordering {
        self.cost.cmp(&other.cost).then(self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Handle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(s: usize, t: usize, edges: &HashMap<usize, Vec<(usize, usize)>>, records: &mut HashMap<usize, Record>) {
    let queue = BinaryHeap::new();

    records.entry(s).or_insert(Record::new()).cost = 0;

    queue.push(records.get(s).to_handle(s));

    while let Some(h) = queue.pop() {
        let v = h.vertex;
        let cost = records.get(v).cost;

        if h.cost > cost {
            // Already updated
            continue;
        }

        if v == t {
            break;
        }

        for edge in edges.get(v) {
            let u = edge.0;
            let new_cost = cost + edge.1;

            let rec = records.entry(u).or_insert(Record::new());
            if rec.cost > new_cost {
                rec.parents.clear();
                rec.parents.push(v);
                rec.cost = new_cost;
                queue.push(rec.to_handle(u));
            } else if rec.cost == new_cost {
                rec.parents.push(v);
            }
        }
    }



}

fn main() {
    let (n, m) = read_cols!(usize, usize);
    let (s, t) = read_cols!(usize, usize);
    let edges = HashMap::new();
    for _ in 0..m {
        let (u, v, d) = read_cols!(usize, usize, usize);
        let vec = edges.entry(u).or_insert(Vec::<(usize, usize)>::new());
        vec.push((v, d));
        let vec = edges.entry(v).or_insert(Vec::<(usize, usize)>::new());
        vec.push((u, d));
    }

    



    let 
    println!("{} {} {}", 5, 2, 7);
}