import {Color, WaterSorting} from "wasm-water-sort";
import { memory } from "wasm-water-sort/water_sort_bg"

const SIZE = 25;
const SPACE = 5;
const EMPTY = '#00000000'
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
waterSorting.init_bottle_with_one_color(Color.Red);
waterSorting.init_bottle_with_two_colors(Color.Blue, Color.Red);
waterSorting.init_bottle_with_three_colors(Color.Yellow, Color.Yellow, Color.Yellow);

const canvas = document.getElementById('water-sorting-canvas');
const ctx = canvas.getContext('2d');

const bottles_count = waterSorting.bottles_count();
canvas.width = (bottles_count + 1)*SIZE;
canvas.height = 5*SIZE;


const drawGame = () => {
    drawBottles(waterSorting.bottles());
    requestAnimationFrame(drawGame);
}

function drawBottles(bottlesPtr) {
const bottles = new Uint8Array(memory.buffer, bottlesPtr, bottles_count * 4);
    for (let i = 0; i < bottles_count; i++) {
        for (let j = 0; j < 4; j++) {
            drawBox(i, 4 - j, bottles[i*4+j])
        }
    }
}

function drawBox(x, y, c) {
    if (c == 0) return;
    ctx.fillStyle = colors[c];

    ctx.fillRect(
        x * (SIZE+SPACE),
        y * SIZE,
        SIZE,
        SIZE
    );
}

requestAnimationFrame(drawGame);