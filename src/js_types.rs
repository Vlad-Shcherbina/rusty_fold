use crate::prelude::*;

// Keep these type definitions in sync with ts/types.ts

#[derive(serde::Serialize)]
struct NamedTask {
    name: String,
    task: Task,
    subdivided_task: Task,
    mesh: Mesh,
}

crate::entry_point!("render_all_tasks", render_all_tasks);
fn render_all_tasks() {
    let start = std::time::Instant::now();
    let mut tasks = vec![];
    for e in project_path("data/tasks").read_dir().unwrap() {
        let path = e.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        eprintln!("{}", name);
        let task = crate::task::Task::parse(&std::fs::read_to_string(path).unwrap());
        let subdivided_task = crate::mesh::subdivide(&task);
        let mesh = Mesh::new(&subdivided_task);
        let t = NamedTask {
            name,
            task,
            subdivided_task,
            mesh,
        };
        tasks.push(t);
    }
    let data = serde_json::to_string_pretty(&tasks).unwrap();
    std::fs::write(project_path("cache/all_tasks.json"), data).unwrap();
    eprintln!("it took {:?}", start.elapsed());
}
