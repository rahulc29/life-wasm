import { Cell, Universe } from "life-wasm";
const CELL_SIZE = 5; // 5 pixels
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const canvas = document.getElementById("life-canvas");
const universe = Universe.new();
const universeWidth = universe.width();
const universeHeight = universe.height();
canvas.height = universe.height() * CELL_SIZE + 1;
const canvasHeight = canvas.height;
console.log("Height : " + canvasHeight);
canvas.width = universe.width() * CELL_SIZE + 1;
const canvasWidth = canvas.width;
console.log("Width : " + canvasWidth);
console.log("Total Pixels  : " + (canvasWidth * canvasHeight));
console.log("Total Cells   : " + (universeWidth * universeHeight));
const context = canvas.getContext("2d");
const drawGrid = (context) => {
    context.beginPath();
    context.strokeStyle = GRID_COLOR;
    for (let i = 0; i < canvasWidth; i++) {
        const xCoordinate = (i + 1) * CELL_SIZE;
        context.moveTo(xCoordinate, 0);
        context.lineTo(xCoordinate, (CELL_SIZE * canvasHeight) + 1);
    }
    for (let j = 0; j < canvasHeight; j++) {
        const yCoordinate = (j + 1) * CELL_SIZE;
        context.moveTo(0, yCoordinate);
        context.lineTo((CELL_SIZE * canvasWidth) + 1, yCoordinate);
    }
};
const drawCells = (context) => {
    for (let j = 0; j < canvasHeight; j++) {
        for (let i = 0; i < canvasWidth; i++) {
            const cell = universe.get(i, j);
            if (cell == Cell.Alive) {
                context.fillStyle = ALIVE_COLOR;
            } else {
                context.fillStyle = DEAD_COLOR;
            }
            context.fillRect(
                (i + 1) * CELL_SIZE, // x coordinate
                (j + 1) * CELL_SIZE, // y coordinate
                CELL_SIZE,           // width
                CELL_SIZE            // height
            );
        }
    }
    context.stroke();
};
const drawDelta = (delta, context) => {
    // TODO : Implement draw by delta
};
// why you no type system? 
let animationId = null;
const renderLoop = () => {
    universe.update();
    const delta = universe.delta();
    drawDelta(delta, context);
    drawGrid(context);
    drawCells(context);
    animationId = requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);
drawGrid(context);
drawCells(context);
const button = document.getElementById("play-pause-button");
const onPlay = () => {
    button.textContent = "⏸"; // the magic of unicode
    renderLoop();
};
const onPause = () => {
    button.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};
button.addEventListener("click", event => {
    if (animationId == null) {
        onPlay();
    } else {
        onPause();
    }
});
canvas.addEventListener("click", event => {
    const clickX = event.clientX;
    const clickY = event.clientY;
    const boundingRect = canvas.getBoundingClientRect();
    const boundX = boundingRect.x;
    const boundY = boundingRect.y;
    const xDiff = clickX - boundX;
    console.log("Delta X : " + xDiff);
    const yDiff = clickY - boundY;
    console.log("Delta Y : " + yDiff);
    const i = Math.min(Math.floor(xDiff / CELL_SIZE), universeWidth - 1);
    const j = Math.min(Math.floor(yDiff / CELL_SIZE), universeHeight - 1);
    console.log("Final Index : (%d, %d)", i, j);
    universe.toggle(i, j);
});