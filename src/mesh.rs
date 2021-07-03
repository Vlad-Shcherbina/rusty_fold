use std::collections::{HashMap, HashSet};
use crate::prelude::*;
use crate::geom::segment_intersection;

fn subdivide_edge(a: &Vec2, b: &Vec2, edges: &[(Vec2, Vec2)]) -> Vec<Vec2> {
    let mut pts = HashSet::new();
    pts.insert(a.clone());
    pts.insert(b.clone());
    for e in edges {
        if let Some(pt) = segment_intersection((&a, &b), (&e.0, &e.1)) {
            pts.insert(pt);
        }
    }
    let mut pts: Vec<Vec2> = pts.into_iter().collect();
    pts.sort();
    if a > b {
        pts.reverse();
    }
    pts
}

fn subdivide_poly(poly: &[Vec2], edges: &[(Vec2, Vec2)]) -> Vec<Vec2> {
    let mut res = vec![];
    for (a, b) in iter_edges(poly) {
        for pt in subdivide_edge(a, b, edges).iter().skip(1) {
            res.push(pt.clone());
        }
    }
    res
}

pub fn subdivide(task: &Task) -> Task {
    let mut skeleton = vec![];
    for e1 in &task.skeleton {
        let pts = subdivide_edge(&e1.0, &e1.1, &task.skeleton);
        for (pt1, pt2) in pts.iter().zip(pts.iter().skip(1)) {
            skeleton.push((pt1.clone(), pt2.clone()));
        }
    }
    Task {
        outer: subdivide_poly(&task.outer, &task.skeleton),
        holes: task.holes.iter().map(|h| subdivide_poly(h, &task.skeleton)).collect(),
        skeleton,
    }
}

#[derive(Debug)]
struct Mesh {
    pts: Vec<Vec2>,
    pt_to_idx: HashMap<Vec2, usize>,
    // polys: Vec<Vec<usize>>,

    pt_idxs_to_half_edge: HashMap<(usize, usize), usize>,
    half_edges: Vec<(usize, usize)>,  // (start, end)
    opposite: Vec<usize>,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl Mesh {
    fn new(edges: &[(Vec2, Vec2)]) -> Mesh {
        let mut pts: HashSet<Vec2> = HashSet::new();
        for (a, b) in edges {
            pts.insert(a.clone());
            pts.insert(b.clone());
        }
        let mut pts: Vec<Vec2> = pts.into_iter().collect();
        pts.sort();

        let pt_to_idx: HashMap<Vec2, usize> = pts.iter().enumerate()
            .map(|(i, pt)| (pt.clone(), i))
            .collect();

        let mut adj: Vec<Vec<usize>> = vec![vec![]; pts.len()];
        for (a, b) in edges {
            let a = pt_to_idx[a];
            let b = pt_to_idx[b];
            adj[a].push(b);
            adj[b].push(a);
        }

        for (aa, pt) in adj.iter_mut().zip(pts.iter()) {
            aa.sort_by_cached_key(|&a| (&pts[a] - pt).angle());
        }

        let mut half_edges = vec![];
        let mut pt_idxs_to_half_edge = HashMap::new();
        for (start, ends) in adj.iter().enumerate() {
            for &end in ends {
                let old = pt_idxs_to_half_edge.insert((start, end), half_edges.len());
                assert!(old.is_none());
                half_edges.push((start, end));
            }
        }

        let mut opposite = vec![usize::max_value(); half_edges.len()];
        for (i, &(start, end)) in half_edges.iter().enumerate() {
            let opp = pt_idxs_to_half_edge[&(end, start)];
            opposite[i] = opp;
        }

        let mut prev = vec![usize::max_value(); half_edges.len()];
        let mut next = vec![usize::max_value(); half_edges.len()];
        for (start, ends) in adj.iter().enumerate() {
            for (&end1, &end2) in ends.iter().zip(ends.iter().skip(1).chain(std::iter::once(&ends[0]))) {
                let he1 = pt_idxs_to_half_edge[&(end1, start)];
                let he2 = pt_idxs_to_half_edge[&(start, end2)];
                next[he1] = he2;
                prev[he2] = he1;
            }
        }

        Mesh {
            pts,
            pt_to_idx,

            pt_idxs_to_half_edge,
            half_edges,
            opposite,
            next,
            prev,
        }
    }
}

#[test]
fn test_mesh() {
    let s = std::fs::read_to_string(
        project_path("data/tasks/A-01.txt")).unwrap();
    let t = Task::parse(&s);
    let t = subdivide(&t);
    let mesh = Mesh::new(&t.skeleton);
    dbg!(mesh);
}
