use crate::prelude::*;

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Task {
    pub outer: Vec<Pt>,
    pub holes: Vec<Vec<Pt>>,
    pub skeleton: Vec<(Pt, Pt)>,
}

impl Task {
    pub fn parse(s: &str) -> Task {
        let mut lines = s.split_terminator('\n');

        let line = lines.next().unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut pos_polys = vec![];
        let mut neg_polys = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap();
            let m: usize = line.trim().parse().unwrap();
            let mut poly = vec![];
            for _ in 0..m {
                let line = lines.next().unwrap();
                poly.push(Pt::parse(line.trim()));
            }
            match area(&poly).cmp(&num_traits::zero()) {
                std::cmp::Ordering::Less => neg_polys.push(poly),
                std::cmp::Ordering::Equal => panic!(),
                std::cmp::Ordering::Greater => pos_polys.push(poly),
            }
            assert_eq!(pos_polys.len(), 1);
        }

        let line = lines.next().unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut skeleton = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap();
            let (a, b) = line.trim().split_once(' ').unwrap();
            skeleton.push((Pt::parse(a), Pt::parse(b)));
        }
        Task {
            outer: pos_polys.pop().unwrap(),
            holes: neg_polys,
            skeleton,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::project_path;

    #[test]
    fn test_parse() {
        let s = std::fs::read_to_string(
            project_path("data/tasks/A-16.txt")).unwrap();
        let p = Task::parse(&s);
        dbg!(p);
    }

    #[test]
    fn parse_all() {
        let entries = project_path("data/tasks").read_dir().unwrap();
        for e in entries {
            let e = e.unwrap();
            dbg!(e.path());
            Task::parse(&std::fs::read_to_string(e.path()).unwrap());
        }
    }
}
