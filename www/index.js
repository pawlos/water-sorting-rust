import {Color, WaterSorting} from "wasm-water-sort";
import { memory } from "wasm-water-sort/water_sort_bg"

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
const colors = [EMPTY, BLUE, RED, GRAY, ORANGE, BROWN, YELLOW, GREEN, MAGENTA];
const waterSorting = WaterSorting.new();
waterSorting.init_bottle_with_four_colors(Color.Red, Color.Magenta, Color.Magenta, Color.Orange);
waterSorting.init_bottle_with_four_colors(Color.Yellow, Color.Brown, Color.Blue, Color.Green);
waterSorting.init_bottle_with_four_colors(Color.Brown, Color.Red, Color.Orange, Color.Red);
waterSorting.init_bottle_with_four_colors(Color.Brown, Color.Blue, Color.Blue, Color.Orange);
waterSorting.init_bottle_with_four_colors(Color.Green, Color.Green, Color.Orange, Color.Yellow);
waterSorting.init_bottle_with_four_colors(Color.Red, Color.Yellow, Color.Magenta, Color.Magenta);
waterSorting.init_bottle_with_four_colors(Color.Blue, Color.Green, Color.Brown, Color.Yellow);
waterSorting.init_empty_bottle();
waterSorting.init_empty_bottle();

const canvas = document.getElementById('water-sorting-canvas');
const ctx = canvas.getContext('2d');

const bottles_count = waterSorting.bottles_count();
canvas.width = PADDING+(bottles_count + 1)*(SIZE+SPACE)+10;
canvas.height = 5*SIZE+10;

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

const mouseClick = (ev) => {
    let point = getMousePos(canvas, ev);
    let x = point.x;
    const within = (el) => el.x1 <= x && el.x2 >= x;
    let index = positions.findIndex(within);
    if (index === -1) return;
    selected[selected.length % 2] = index;

    if (selected.length === 2) {
        waterSorting.pour(selected[0], selected[1]);
        selected.splice(0, 2);
    }
}
canvas.onclick = mouseClick;

const drawGame = () => {
    clear();
    if (waterSorting.win()) {
        drawWin();
    } else {
        drawBottles(waterSorting.bottles());
        requestAnimationFrame(drawGame);
    }
}

const drawWin = () => {
    ctx.fillStyle = colors[4];
    ctx.font = "40px serif"
    ctx.fillText("You win!", 50,50, 100);
}

const drawBottles = (bottlesPtr) => {
    const bottles = new Uint8Array(memory.buffer, bottlesPtr, bottles_count * 4);
    for (let i = 0; i < bottles_count; i++) {
        drawBottle(i);
        for (let j = 0; j < 4; j++) {
            drawBox(i, 4 - j, bottles[i*4+j])
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
    let padding = 0;
    if (selected[0] === x)
        padding = SELECTED_PADDING;
    ctx.moveTo(PADDING + x * (SIZE + SPACE)-1, SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE + SPACE)-1, 5*SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE+SPACE)+SIZE+1, 5*SIZE+1+padding);
    ctx.lineTo(PADDING + x * (SIZE+SPACE)+SIZE+1, SIZE+1+padding);
    ctx.stroke();
}

const drawBox = (x, y, c) => {
    ctx.fillStyle = colors[c];
    let padding = 0;
    if (selected[0] === x)
        padding = SELECTED_PADDING;
    ctx.fillRect(
        PADDING + x * (SIZE+SPACE),
        y * SIZE + padding,
        SIZE,
        SIZE
    );
}

requestAnimationFrame(drawGame);