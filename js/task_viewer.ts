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
        }
    };
}

function render_task(tasks: NamedTask[], task_no: number) {
    let task = tasks[task_no];
    let canvas = document.querySelector('canvas')!;
    canvas.width = canvas.width;  // clear
    let ctx = canvas.getContext("2d")!;

    let x1 = Infinity;
    let y1 = Infinity;
    let x2 = -Infinity;
    let y2 = -Infinity;
    for (let edge of task.task.skeleton) {
        for (let pt of edge) {
            let [x, y] = pt;
            x1 = Math.min(x1, x);
            y1 = Math.min(y1, y);
            x2 = Math.max(x2, x);
            y2 = Math.max(y2, y);
        }
    }
    let border = 3.5;
    let scale = Math.min(
        (canvas.width - 2 * border) / (x2 - x1),
        (canvas.height - 2 * border) / (y2 - y1));

    function transform([x, y]: Pt): Pt {
        return [
            border + (x - x1) * scale,
            border + (y - y1) * scale,
        ];
    }

    ctx.lineWidth = 1;
    ctx.strokeStyle = 'black';
    for (let edge of task.task.skeleton) {
        ctx.beginPath();
        ctx.moveTo(...transform(edge[0]));
        ctx.lineTo(...transform(edge[1]));
        ctx.stroke();
    }

    ctx.strokeStyle = 'blue';
    for (let edge of task.task.subdivided_skeleton) {
        ctx.beginPath();
        let [x, y] = transform(edge[0]);
        ctx.moveTo(x + Math.random() * 5, y + Math.random() * 5);
        [x, y] = transform(edge[1]);
        ctx.lineTo(x + Math.random() * 5, y + Math.random() * 5);
        ctx.stroke();
    }

    let caption = document.getElementById('caption')!;
    let sz = Math.max(x2 - x1, y2 - y1);
    caption.innerText = `${task.name} (${task_no + 1}/${tasks.length}), size=${sz}`;

    ctx.lineWidth = 2;
    ctx.strokeStyle = '#0a0';
    ctx.beginPath();
    for (let pt of task.task.outer) {
        ctx.lineTo(...transform(pt));
    }
    ctx.closePath();
    ctx.stroke();

    ctx.strokeStyle = '#a00';
    for (let hole of task.task.holes) {
        ctx.beginPath();
        for (let pt of hole) {
            ctx.lineTo(...transform(pt));
        }
        ctx.closePath();
        ctx.stroke();
    }
}

main();
