import { Automaton } from 'wasm-game-of-life';
import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg';

const cell_size = 5;
const grid_color = '#cccccc';
const dead_color = '#ffffff';
const alive_color = '#000000';

let width = 64;
let height = 64;
let generation = 0;
const automaton = Automaton.new(width, height);

const canvas = document.getElementById('game-canvas');
canvas.height = (cell_size + 1) * height + 1;
canvas.width = (cell_size + 1) * width + 1;

const context = canvas.getContext('2d');

let animationId = null;
let stepSize = 1;
let randomizePercent = 50;

const render = () => {
  automaton.step(stepSize);

  drawGrid();
  drawCells();

  incrementGeneration();

  animationId = requestAnimationFrame(render);
};

const drawGrid = () => {
  context.beginPath();
  context.strokeStyle = grid_color;

  for (let i = 0; i <= width; i++) {
    context.moveTo(i * (cell_size + 1) + 1, 0);
    context.lineTo(i * (cell_size + 1) + 1, (cell_size + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    context.moveTo(0, j * (cell_size + 1) + 1);
    context.lineTo((cell_size + 1) * width + 1, j * (cell_size + 1) + 1);
  }

  context.stroke();
};

const drawCells = () => {
  const cellsPtr = automaton.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  context.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      context.fillStyle = cells[row * width + col] === 0
        ? dead_color
        : alive_color;

      context.fillRect(
        col * (cell_size + 1) + 1,
        row * (cell_size + 1) + 1,
        cell_size,
        cell_size
      );
    }
  }

  context.stroke();
};

canvas.addEventListener('click', (e) => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;
  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;
  const row = Math.min(Math.floor(canvasTop / (cell_size + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (cell_size + 1)), width - 1);

  automaton.toggle_cell(row, col);

  drawGrid();
  drawCells();
})

const generationSpan = document.getElementById('generation');
const incrementGeneration = () => {
  generation += stepSize;
  generationSpan.textContent = generation.toString();
};
const resetGeneration = () => {
  generation = 0;
  generationSpan.textContent = generation.toString();
};

const playPauseButton = document.getElementById('play-pause');
const isPaused = () => animationId === null;

const play = () => {
  playPauseButton.textContent = 'Pause';
  render();
};

const pause = () => {
  playPauseButton.textContent = 'Start';
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener('click', (e) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

document.getElementById('step').addEventListener('click', (e) => {
  automaton.step(stepSize);
  drawGrid();
  drawCells();

  incrementGeneration();
});

let stepSizeInput = document.getElementById('step-size');
stepSizeInput.addEventListener('change', (e) => {
  stepSize = parseInt(stepSizeInput.value, 10);
});

document.getElementById('reset').addEventListener('click', (e) => {
  automaton.set_all_cells(0);
  drawGrid();
  drawCells();

  resetGeneration();
});

document.getElementById('randomize').addEventListener('click', (e) => {
  automaton.randomize_cells(randomizePercent);
  drawGrid();
  drawCells();

  resetGeneration();
});

let randomizePercentInput = document.getElementById('randomize-percent');
randomizePercentInput.addEventListener('change', (e) => {
  randomizePercent = randomizePercentInput.value;
});

drawGrid();
drawCells();
pause();
