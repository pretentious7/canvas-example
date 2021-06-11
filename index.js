// For more comments about what's going on here, check out the `hello_world`
// example.
/*
import('./pkg')
  .catch(console.error);
*/

const rust = import('./pkg');

let time = document.getElementById("time");

rust
  .then(m => {
    let start = performance.now();
    m.render();
    let end = performance.now();
    time.textContent = end-start;
  }); 