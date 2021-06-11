
// For more comments about what's going on here, check out the `hello_world`
//import('./pkg')
//  .catch(console.error);

const wasm = import('./pkg');

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');
let time = document.getElementById('time');

wasm.then(m => {
  let start = performance.now();
  let imageData = m.run();
  ctx.putImageData(imageData,0,0);
  let end = performance.now();
  time.textContent = "" + end-start;
});

