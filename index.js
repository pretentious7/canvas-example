const wasm = import('./pkg');

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');
let time = document.getElementById('time');

const WIDTH = canvas.width;
const HEIGHT = canvas.height;
const START_X = 0.300283;
const START_Y =  -0.48857;
const WINDOW = 0.01;

wasm.then(m => {

  let start = performance.now();

  //rust function returns a ImageData Object
  let imageData = m.run(START_X, START_Y, WIDTH, HEIGHT, WINDOW); 

  ctx.putImageData(imageData,0,0);
  let end = performance.now();

  time.textContent = "" + end-start;
});

