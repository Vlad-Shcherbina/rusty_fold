// Keep these type definitions in sync with src/js_types.rs

type Pt = [number, number]
type Poly = Pt[]

interface Task {
    silhouette: Poly[],
    skeleton: [Pt, Pt][],
}

interface NamedTask {
    name: string,
    task: Task,
}
