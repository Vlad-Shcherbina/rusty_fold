use std::collections::{HashMap, HashSet};
use tyndex::{Tyndex, TyVec};
use crate::prelude::*;
use crate::geom::segment_intersection;

fn subdivide_edge(a: &Pt, b: &Pt, edges: &[(Pt, Pt)]) -> Vec<Pt> {
    let mut pts = HashSet::new();
    pts.insert(a.clone());
    pts.insert(b.clone());
    for e in edges {
        if let Some(pt) = segment_intersection((a, b), (&e.0, &e.1)) {
            pts.insert(pt);
        }
    }
    let mut pts: Vec<Pt> = pts.into_iter().collect();
    pts.sort();
    if a > b {
        pts.reverse();
    }
    pts
}

fn subdivide_poly(poly: &[Pt], edges: &[(Pt, Pt)]) -> Vec<Pt> {
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

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, Debug)]
pub struct HalfEdge(u32);

impl Tyndex for HalfEdge {
    fn from_index(i: usize) -> Self {
        HalfEdge(i.try_into().unwrap())
    }

    fn to_index(self) -> usize {
        self.0 as usize
    }
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Mesh {
    pts: Vec<Pt>,
    #[serde(skip)]
    pt_to_idx: HashMap<Pt, usize>,
    // polys: Vec<Vec<usize>>,

    #[serde(skip)]
    pt_idxs_to_half_edge: HashMap<(usize, usize), HalfEdge>,
    half_edges: TyVec<HalfEdge, (usize, usize)>,
    opposite: TyVec<HalfEdge, HalfEdge>,
    prev: TyVec<HalfEdge, HalfEdge>,
    next: TyVec<HalfEdge, HalfEdge>,

    he_poly: TyVec<HalfEdge, usize>,
    poly_he: Vec<HalfEdge>,
    poly_real: Vec<bool>,
}

impl Mesh {
    pub fn new(task: &Task) -> Mesh {
        let mut pts: HashSet<Pt> = HashSet::new();
        for (a, b) in &task.skeleton {
            pts.insert(a.clone());
            pts.insert(b.clone());
        }
        let mut pts: Vec<Pt> = pts.into_iter().collect();
        pts.sort();

        let pt_to_idx: HashMap<Pt, usize> = pts.iter().enumerate()
            .map(|(i, pt)| (pt.clone(), i))
            .collect();

        let mut adj: Vec<Vec<usize>> = vec![vec![]; pts.len()];
        for (a, b) in &task.skeleton {
            let a = pt_to_idx[a];
            let b = pt_to_idx[b];
            adj[a].push(b);
            adj[b].push(a);
        }

        for (aa, pt) in adj.iter_mut().zip(pts.iter()) {
            aa.sort_by_cached_key(|&a| (&pts[a] - pt).angle());
        }

        let mut half_edges = TyVec::<HalfEdge, _>::from_raw(vec![]);
        let mut pt_idxs_to_half_edge = HashMap::new();
        for (start, ends) in adj.iter().enumerate() {
            for &end in ends {
                let he = half_edges.push_and_idx((start, end));
                let old = pt_idxs_to_half_edge.insert((start, end), he);
                assert!(old.is_none());
            }
        }

        let opposite = TyVec::from_raw(half_edges.raw.iter()
            .map(|&(start, end)|
                pt_idxs_to_half_edge[&(end, start)])
            .collect());

        let mut prev = TyVec::from_raw(vec![HalfEdge(u32::max_value()); half_edges.raw.len()]);
        let mut next = TyVec::from_raw(vec![HalfEdge(u32::max_value()); half_edges.raw.len()]);
        for (start, ends) in adj.iter().enumerate() {
            for (&end1, &end2) in ends.iter().zip(ends.iter().skip(1).chain(std::iter::once(&ends[0]))) {
                let he1 = pt_idxs_to_half_edge[&(end2, start)];
                let he2 = pt_idxs_to_half_edge[&(start, end1)];
                next[he1] = he2;
                prev[he2] = he1;
            }
        }

        let mut he_poly: TyVec<HalfEdge, Option<usize>> =
            TyVec::from_raw(vec![None; half_edges.raw.len()]);
        let mut poly_he = vec![];

        for e in (0..half_edges.raw.len()).map(HalfEdge::from_index) {
            if he_poly[e].is_some() {
                continue;
            }
            let mut e2 = e;
            loop {
                he_poly[e2] = Some(poly_he.len());
                e2 = next[e2];
                if e2 == e {
                    break;
                }
            }
            poly_he.push(e);
        }
        let he_poly = TyVec::from_raw(he_poly.raw.into_iter().map(Option::unwrap).collect());

        let mut poly_real = vec![true; poly_he.len()];
        let e = pt_idxs_to_half_edge[&(pt_to_idx[&task.outer[1]], pt_to_idx[&task.outer[0]])];
        poly_real[he_poly[e]] = false;
        for hole in &task.holes {
            let e = pt_idxs_to_half_edge[&(pt_to_idx[&hole[1]], pt_to_idx[&hole[0]])];
            poly_real[he_poly[e]] = false;
        }

        Mesh {
            pts,
            pt_to_idx,

            pt_idxs_to_half_edge,
            half_edges,
            opposite,
            next,
            prev,

            he_poly,
            poly_he,
            poly_real,
        }
    }
}

#[test]
fn test_mesh() {
    let s = std::fs::read_to_string(
        project_path("data/tasks/A-01.txt")).unwrap();
    let t = Task::parse(&s);
    let t = subdivide(&t);
    let mesh = Mesh::new(&t);
    dbg!(mesh);
}
