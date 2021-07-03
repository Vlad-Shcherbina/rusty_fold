use std::collections::HashSet;
use crate::prelude::*;
use crate::geom::segment_intersection;

pub fn subdivide_edges(edges: &[(Vec2, Vec2)]) -> Vec<(Vec2, Vec2)> {
    let mut result = vec![];
    for e1 in edges {
        let mut pts = HashSet::new();
        pts.insert(e1.0.clone());
        pts.insert(e1.1.clone());
        for e2 in edges {
            if let Some(pt) = segment_intersection((&e1.0, &e1.1), (&e2.0, &e2.1)) {
                pts.insert(pt);
            }
        }
        let mut pts: Vec<Vec2> = pts.into_iter().collect();
        pts.sort();
        for (pt1, pt2) in pts.iter().zip(pts.iter().skip(1)) {
            result.push((pt1.clone(), pt2.clone()));
        }
    }
    result
}
