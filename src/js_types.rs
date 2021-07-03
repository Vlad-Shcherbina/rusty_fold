use crate::prelude::*;
use num_traits::cast::ToPrimitive;

// Keep these type definitions in sync with ts/types.ts

#[derive(serde::Serialize)]
pub struct NamedTask {
    name: String,
    task: Task,
}

type Pt = (f64, f64);
type Poly = Vec<Pt>;

#[derive(serde::Serialize)]
pub struct Task {
    silhouette: Vec<Poly>,
    skeleton: Vec<(Pt, Pt)>,
}

fn vec2_to_pt(p: &Vec2) -> Pt {
    (p.x.to_f64().unwrap(), p.y.to_f64().unwrap())
}

impl From<&crate::task::Task> for Task {
    fn from(p: &crate::task::Task) -> Self {
        let silhouette = p.silhouette.iter()
        .map(|poly|
            poly.iter().map(vec2_to_pt).collect()
        )
        .collect();
        let skeleton = p.skeleton.iter()
        .map(|(a, b)| (vec2_to_pt(a), vec2_to_pt(b)))
        .collect();
        Task {
            silhouette,
            skeleton,
        }
    }
}

crate::entry_point!("render_all_tasks", render_all_tasks);
fn render_all_tasks() {
    let mut tasks = vec![];
    for e in project_path("data/tasks").read_dir().unwrap() {
        let path = e.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let task = crate::task::Task::parse(&std::fs::read_to_string(path).unwrap());
        let task = Task::from(&task);
        let task = NamedTask { name, task };
        tasks.push(task);
    }
    let data = serde_json::to_string_pretty(&tasks).unwrap();
    std::fs::write(project_path("cache/all_tasks.json"), data).unwrap();
}
