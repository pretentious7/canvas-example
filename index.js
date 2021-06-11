const wasm = import('./pkg');

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');
let time = document.getElementById('time');

wasm.then(m => {

  let start = performance.now();
  let imageData = m.run(); //rust function returns a ImageData Object
  ctx.putImageData(imageData,0,0);
  let end = performance.now();

  time.textContent = "" + end-start;
});

