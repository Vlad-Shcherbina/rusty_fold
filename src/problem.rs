use crate::prelude::*;

#[derive(Debug)]
pub struct Problem {
    pub silhouette: Vec<Vec<Vec2>>,
    pub skeleton: Vec<(Vec2, Vec2)>,
}

impl Problem {
    pub fn parse(s: &str) -> Problem {
        let mut lines = s.split_terminator('\n');

        let line = lines.next().unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut silhouette = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap();
            let m: usize = line.trim().parse().unwrap();
            let mut poly = vec![];
            for _ in 0..m {
                let line = lines.next().unwrap();
                poly.push(Vec2::parse(line.trim()));
            }
            silhouette.push(poly);
        }

        let line = lines.next().unwrap();
        let n: usize = line.trim().parse().unwrap();
        let mut skeleton = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap();
            let (a, b) = line.trim().split_once(' ').unwrap();
            skeleton.push((Vec2::parse(a), Vec2::parse(b)));
        }
        Problem {
            silhouette,
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
        let p = Problem::parse(&s);
        dbg!(p);
    }

    #[test]
    fn parse_all() {
        let entries = project_path("data/tasks").read_dir().unwrap();
        for e in entries {
            let e = e.unwrap();
            dbg!(e.path());
            Problem::parse(&std::fs::read_to_string(e.path()).unwrap());
        }
    }
}