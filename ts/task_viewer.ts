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
    let caption = document.getElementById('caption')!;
    caption.innerText = `${task.name} (${task_no + 1}/${tasks.length})`;
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
    let scale = Math.min(canvas.width / (x2 - x1), canvas.height / (y2 - y1));

    function transform([x, y]: Pt): Pt {
        return [
            (x - x1) * scale,
            (y - y1) * scale,
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
}

main();
