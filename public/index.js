import { Automaton } from 'wasm-game-of-life';
import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg';

const CELL_SIZE = 5;
const GRID_COLOR = '#cccccc';
const DEAD_COLOR = '#ffffff';
const ALIVE_COLOR = '#000000';

const automaton = Automaton.new(64, 64);
const width = automaton.width();
const height = automaton.height();

const canvas = document.getElementById('game-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const context = canvas.getContext('2d');

let animationId = null;
let stepSize = 1;
let randomizePercent = 50;

const render = () => {
  automaton.step(stepSize);
  updateGeneration();

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(render);
};

const drawGrid = () => {
  context.beginPath();
  context.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++) {
    context.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    context.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    context.moveTo(0, j * (CELL_SIZE + 1) + 1);
    context.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
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
        ? DEAD_COLOR
        : ALIVE_COLOR;

      context.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
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
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  automaton.cell_toggle(row, col);

  drawGrid();
  drawCells();
})

const playPauseButton = document.getElementById('play-pause');
const isPaused = () => animationId === null;

const play = () => {
  playPauseButton.innerText = 'Pause';
  render();
};

const pause = () => {
  playPauseButton.innerText = 'Start';
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

const generationSpan = document.getElementById('generation');
const updateGeneration = () => {
  generationSpan.innerText = automaton.generation();
};

let stepSizeInput = document.getElementById('step-size');
stepSizeInput.addEventListener('change', (e) => {
  stepSize = stepSizeInput.value;
});

document.getElementById('reset').addEventListener('click', (e) => {
  automaton.set_all_cells(0);
  drawGrid();
  drawCells();
  updateGeneration();
});

document.getElementById('randomize').addEventListener('click', (e) => {
  automaton.randomize_cells(randomizePercent);
  drawGrid();
  drawCells();
  updateGeneration();
});

let randomizePercentInput = document.getElementById('randomize-percent');
randomizePercentInput.addEventListener('change', (e) => {
  randomizePercent = randomizePercentInput.value;
});

drawGrid();
drawCells();
pause();
