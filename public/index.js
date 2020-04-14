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

const render = () => {
  automaton.step();
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

const generation = document.getElementById('generation');
const updateGeneration = () => {
  generation.textContent = automaton.generation();
};

drawGrid();
drawCells();
pause();
