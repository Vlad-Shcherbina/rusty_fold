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

#[derive(serde::Serialize)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Vertex(u32);

impl Tyndex for Vertex {
    fn from_index(i: usize) -> Self {
        Self(i.try_into().unwrap())
    }
    fn to_index(self) -> usize {
        self.0 as usize
    }
}

#[derive(serde::Serialize)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HalfEdge(u32);

impl Tyndex for HalfEdge {
    fn from_index(i: usize) -> Self {
        Self(i.try_into().unwrap())
    }
    fn to_index(self) -> usize {
        self.0 as usize
    }
}

#[derive(serde::Serialize)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Poly(u32);

impl Tyndex for Poly {
    fn from_index(i: usize) -> Self {
        Self(i.try_into().unwrap())
    }
    fn to_index(self) -> usize {
        self.0 as usize
    }
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Mesh {
    pts: TyVec<Vertex,Pt>,
    #[serde(skip)]
    pt_to_idx: HashMap<Pt, Vertex>,

    #[serde(skip)]
    pt_idxs_to_half_edge: HashMap<(Vertex, Vertex), HalfEdge>,
    half_edges: TyVec<HalfEdge, (Vertex, Vertex)>,
    opposite: TyVec<HalfEdge, HalfEdge>,
    prev: TyVec<HalfEdge, HalfEdge>,
    next: TyVec<HalfEdge, HalfEdge>,

    he_poly: TyVec<HalfEdge, Poly>,
    poly_he: TyVec<Poly, HalfEdge>,
    poly_real: TyVec<Poly, bool>,
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
        let pts = TyVec::<Vertex, _>::from_raw(pts);

        let pt_to_idx: HashMap<Pt, Vertex> = pts.enum_ref()
            .map(|(i, pt)| (pt.clone(), i))
            .collect();

        let mut adj: TyVec<Vertex, Vec<Vertex>> = TyVec::from_raw(vec![vec![]; pts.raw.len()]);
        for (a, b) in &task.skeleton {
            let a = pt_to_idx[a];
            let b = pt_to_idx[b];
            adj[a].push(b);
            adj[b].push(a);
        }

        for (aa, pt) in adj.raw.iter_mut().zip(pts.raw.iter()) {
            aa.sort_by_cached_key(|&a| (&pts[a] - pt).angle());
        }

        let mut half_edges = TyVec::<HalfEdge, _>::from_raw(vec![]);
        let mut pt_idxs_to_half_edge = HashMap::new();
        for (start, ends) in adj.enum_ref() {
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
        for (start, ends) in adj.enum_ref() {
            for (&end1, &end2) in ends.iter().zip(ends.iter().skip(1).chain(std::iter::once(&ends[0]))) {
                let he1 = pt_idxs_to_half_edge[&(end2, start)];
                let he2 = pt_idxs_to_half_edge[&(start, end1)];
                next[he1] = he2;
                prev[he2] = he1;
            }
        }

        let mut he_poly: TyVec<HalfEdge, Option<Poly>> =
            TyVec::from_raw(vec![None; half_edges.raw.len()]);
        let mut poly_he = TyVec::<Poly, HalfEdge>::from_raw(vec![]);

        for e in (0..half_edges.raw.len()).map(HalfEdge::from_index) {
            if he_poly[e].is_some() {
                continue;
            }
            let p = poly_he.push_and_idx(e);
            let mut e2 = e;
            loop {
                he_poly[e2] = Some(p);
                e2 = next[e2];
                if e2 == e {
                    break;
                }
            }
        }
        let he_poly = TyVec::from_raw(he_poly.raw.into_iter().map(Option::unwrap).collect());

        let mut poly_real = TyVec::<Poly, bool>::from_raw(vec![true; poly_he.raw.len()]);
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
