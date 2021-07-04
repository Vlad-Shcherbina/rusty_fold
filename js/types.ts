// Keep these type definitions in sync with all Rust types that implement
// serde::Serialize.

type Pt = [number, number]
type Poly = Pt[]

interface Task {
    outer: Poly,
    holes: Poly[],
    skeleton: [Pt, Pt][],
}

interface NamedTask {
    name: string,
    task: Task,
    subdivided_task: Task,
}
