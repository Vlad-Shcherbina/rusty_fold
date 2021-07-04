import assert from './assert.js';

async function main() {
    let res = await fetch('/cache/all_tasks.json');
    assert(res.status == 200);
    let tasks: NamedTask[] = await res.json();

    let task_no = 0;
    render_task(tasks, task_no);

    document.onkeydown = (e) => {
        switch (e.code) {
            case 'ArrowLeft': {
                e.preventDefault();
                task_no -= 1;
                if (task_no < 0) {
                    task_no = 0;
                }
                render_task(tasks, task_no);
                break;
            }
            case 'ArrowRight': {
                e.preventDefault();
                task_no += 1;
                if (task_no >= tasks.length) {
                    task_no = tasks.length - 1;
                }
                render_task(tasks, task_no);
                break;
            }
            default:
                render_task(tasks, task_no);
        }
    };
}

function render_task(tasks: NamedTask[], task_no: number) {
    let task = tasks[task_no];
    let t = task.subdivided_task;
    let mesh = task.mesh;
    let canvas = document.querySelector('canvas')!;
    canvas.width = canvas.width;  // clear
    let ctx = canvas.getContext("2d")!;

    let x1 = Infinity;
    let y1 = Infinity;
    let x2 = -Infinity;
    let y2 = -Infinity;
    for (let edge of t.skeleton) {
        for (let pt of edge) {
            let [x, y] = pt;
            x1 = Math.min(x1, x);
            y1 = Math.min(y1, y);
            x2 = Math.max(x2, x);
            y2 = Math.max(y2, y);
        }
    }
    let border = 100.5;
    let scale = Math.min(
        (canvas.width - 2 * border) / (x2 - x1),
        (canvas.height - 2 * border) / (y2 - y1));

    function transform([x, y]: Pt): Pt {
        return [
            border + (x - x1) * scale,
            border + (y2 - y) * scale,
        ];
    }
    function perturb([x, y]: [number, number]): [number, number] {
        return [x + 7 * (Math.random() - 0.5), y + 7 * (Math.random() - 0.5)];
    }

    console.dir(mesh.poly_real);

    ctx.globalAlpha = 0.3;
    ctx.fillStyle = 'gray';
    mesh.poly_he.forEach((he, i) => {
        if (mesh.poly_real[i]) {
            let he2 = he;
            ctx.beginPath();
            while (true) {
                ctx.lineTo(...transform(mesh.pts[mesh.half_edges[he2][0]]));
                he2 = mesh.next[he2];
                if (he2 == he) {
                    break;
                }
            }
            ctx.fill();
        }
    });
    ctx.globalAlpha = 1.0;

    ctx.lineWidth = 1;
    ctx.strokeStyle = 'black';
    for (let edge of t.skeleton) {
        ctx.beginPath();
        ctx.moveTo(...transform(edge[0]));
        ctx.lineTo(...transform(edge[1]));
        ctx.stroke();
    }

    ctx.lineWidth = 2;
    ctx.strokeStyle = '#0a0';
    ctx.beginPath();
    for (let pt of t.outer) {
        ctx.lineTo(...transform(pt));
    }
    ctx.closePath();
    ctx.stroke();

    ctx.beginPath();
    for (let pt of t.outer) {
        ctx.arc(...transform(pt), 2, 0, 2 * Math.PI);
    }
    ctx.stroke();

    ctx.strokeStyle = '#a00';
    for (let hole of t.holes) {
        ctx.beginPath();
        for (let pt of hole) {
            ctx.lineTo(...transform(pt));
        }
        ctx.closePath();
        ctx.stroke();
    }

    ctx.lineWidth = 0.5;
    ctx.strokeStyle = '#00a';
    mesh.half_edges.forEach(([i, j], idx) => {
        if (!mesh.poly_real[mesh.he_poly[idx]]) {
            return;
        }
        let [x1, y1] = mesh.pts[i];
        let [x2, y2] = mesh.pts[j];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let q = 0.1;
        ctx.beginPath();
        ctx.moveTo(...transform([x1 + 0.5 * dx - q * dy, y1 + 0.5 * dy + q * dx]));
        ctx.lineTo(...transform([x2 - q * dx - q * dy, y2 - q * dy + q * dx]));
        ctx.stroke();
    });

    let caption = document.getElementById('caption')!;
    let sz = Math.max(x2 - x1, y2 - y1);
    caption.innerText = `${task.name} (${task_no + 1}/${tasks.length}), size=${sz}`;
}

main();
