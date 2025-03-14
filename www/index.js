import {Color, WaterSorting, WaterSolver} from "wasm-water-sort";
import { memory } from "../pkg/water_sort_bg.wasm";

const SIZE = 25;
const SPACE = 10;
const PADDING = 5;
const SELECTED_PADDING = -5;
const EMPTY = '#FFFFFF'
const BLUE = '#000080';
const RED = '#fb0606';
const GRAY = '#808080';
const ORANGE = '#F08000'
const BROWN = '#7b2525'
const YELLOW = '#F0F000'
const GREEN = '#008000'
const MAGENTA = '#7f1894'
const LIME = '#7aa402'
const TEAL = '#55b08d'
const PURPLE = '#ab64d4'
const LIGHTBLUE ='#2688ab'
const PEACH = '#cb9486'
const OLIVE = '#194e24'
const colors = [EMPTY, BLUE, RED, GRAY, ORANGE, BROWN, YELLOW, GREEN, MAGENTA, LIME, TEAL, PURPLE, LIGHTBLUE, PEACH, OLIVE];
const waterSorting = WaterSorting.new();
const success = new Audio('success.mp3');

const initialize = (w) => {
    w.init_bottle_with_four_colors(Color.Red, Color.Red, Color.Orange, Color.Blue);
    w.init_bottle_with_four_colors(Color.Peach, Color.Blue, Color.Peach, Color.Orange);
    w.init_bottle_with_four_colors(Color.Peach, Color.Blue, Color.Red, Color.Peach);
    w.init_bottle_with_four_colors(Color.Orange, Color.Teal, Color.Red, Color.Teal);
    w.init_bottle_with_four_colors(Color.Blue, Color.Teal, Color.Teal, Color.Orange);
    w.init_empty_bottle();
    w.init_empty_bottle();
};
initialize(waterSorting);

const canvas = document.getElementById('water-sorting-canvas');
const undo_btn = document.getElementById('undo-btn');
const reset_btn = document.getElementById('reset-btn');
const solve_btn = document.getElementById('solve-btn');
const ctx = canvas.getContext('2d');

const bottles_count = waterSorting.bottles_count();
canvas.width = PADDING+(bottles_count + 1)*(SIZE+SPACE)+SPACE;
canvas.height = 5*SIZE+SPACE;

const selected = [];

const positions = [];
for(let i = 0; i<bottles_count;i++)
{
    let x = PADDING + i * (SIZE+SPACE);
    positions.push({'x1': x,'x2':x+SIZE});
}

const getMousePos = (canvas, evt) => {
    const rect = canvas.getBoundingClientRect();
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}

const mouseMove = (ev) => {
    let point = getMousePos(canvas, ev);
    let x = point.x;
    const within = (el) => el.x1 <= x && el.x2 >= x;
    if (positions.some(within))
        canvas.style.cursor = 'pointer';
    else
        canvas.style.cursor = 'default';
}
canvas.onmousemove = mouseMove;

let undo_requested = false;
const undo = () => {
    undo_requested = true;
}
undo_btn.onclick = undo;

let reset_requested = false;
const reset = () => {
    reset_requested = true;
}
reset_btn.onclick = reset;

const perform_move = (moves, index) => {
    if (index < 0 || index >= moves.length) return;
    const move = moves[index];
    const from = move.from;
    const to = move.to;
    waterSorting.pour(from, to);
    setTimeout(perform_move, 1000, moves, ++index);
}

const solve = (ws) => {
    const waterSolver = WaterSolver.new(ws);
    const pours = waterSolver.solution(20);
    const solutions = new Uint8Array(memory.buffer, pours,  4);
    const count = solutions[0];
    const moves = new Uint8Array(memory.buffer, pours,
    /* 4 B * 2 position per pour + 1 for count */ 4 * count * 2 + 1);
    let solution_moves = [];
    for (let i = 1; i < count*2; i+=2) {
        solution_moves.push({from: moves[i*4], to: moves[(i+1)*4]});
    }
    setTimeout(perform_move, 100, solution_moves, 0);
}

solve_btn.onclick = () => solve(waterSorting);

const mouseClick = (ev) => {
    if (waterSorting.win()) return;
    let point = getMousePos(canvas, ev);
    let x = point.x;
    const within = (el) => el.x1 <= x && el.x2 >= x;
    let index = positions.findIndex(within);
    if (index === -1) return;
    if (selected.includes(index))
        selected.splice(0, 1);
    else
        selected[selected.length % 2] = index;


}
canvas.onclick = mouseClick;

const drawGame = () => {
    function perform_undo() {
        undo_requested = false;
        selected.splice(0, 2);
        waterSorting.undo();
    }

    function perform_pouring() {
        waterSorting.pour(selected[0], selected[1]);
        selected.splice(0, 2);
    }
    function perform_reset() {
        reset_requested = false;
        waterSorting.reset();
        initialize(waterSorting);
    }

    waterSorting.undo_available() ? undo_btn.removeAttribute("disabled") : undo_btn.setAttribute("disabled", "disabled");
    clear();

    if (reset_requested) {
        perform_reset();
    }

    if (undo_requested) {
        perform_undo();
    }
    else if (selected.length === 2) {
        perform_pouring();
    }
    if (waterSorting.win()) {
        drawWin();
        success.play().then(() => setTimeout(() => reset_requested = true, 3000));
    } else {
        drawBottles(waterSorting.bottles());
    }
    requestAnimationFrame(drawGame);
}

const drawWin = () => {
    ctx.fillStyle = colors[4];
    ctx.font = "40px serif"
    const win = "You win!";
    const measure = ctx.measureText(win);
    ctx.fillText(win, (canvas.width - measure.width) / 2,canvas.height - 20, measure.width);
}

const drawBottles = (bottlesPtr) => {
    const bottles = new Uint8Array(memory.buffer, bottlesPtr, bottles_count * 4);
    for (let i = 0; i < bottles_count; i++) {
        drawBottle(i);
        for (let j = 0; j < 4; j++) {
            drawFluid(i, 4 - j, bottles[i*4+j])
        }
    }
}

const clear = () => {
    ctx.fillStyle = colors[0];
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

const drawBottle = (x) => {
    ctx.beginPath();
    ctx.strokeStyle = '#000';
    let padding = selected[0] === x ? SELECTED_PADDING: 0;
    ctx.moveTo(PADDING + x * (SIZE + SPACE)-1, SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE + SPACE)-1, 5*SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE+SPACE)+SIZE+1, 5*SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE+SPACE)+SIZE+1, SIZE+1+padding);
    ctx.stroke();
}

const drawFluid = (x, y, c) => {
    ctx.fillStyle = colors[c];
    let padding = selected[0] === x ? SELECTED_PADDING : 0;

    ctx.fillRect(
        PADDING + x * (SIZE+SPACE),
        y * SIZE + padding,
        SIZE,
        SIZE
    );
}

requestAnimationFrame(drawGame);
