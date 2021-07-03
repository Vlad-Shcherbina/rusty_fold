// Keep these type definitions in sync with src/js_types.rs

type Pt = [number, number]
type Poly = Pt[]

interface Task {
    outer: Poly,
    holes: Poly[],
    skeleton: [Pt, Pt][],
    subdivided_skeleton: [Pt, Pt][],
}

interface NamedTask {
    name: string,
    task: Task,
}