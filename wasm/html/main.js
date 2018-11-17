var canvas = document.getElementById('canvas');
var context = canvas.getContext("2d");

function draw_circle(x, y, r) {
  context.beginPath();
  context.arc(x, y, r, 0, 2 * Math.PI);
  context.stroke();
}

function loadWasm(callback) {
  let importObject = {
    env: {
      draw_circle: draw_circle
    }
  };
  fetch('/target/wasm32-unknown-unknown/release/wasm.wasm').then(
    response => response.arrayBuffer()
  ).then(bytes =>
      WebAssembly.instantiate(bytes, importObject)
  ).then(callback).catch(err => console.log("Failed to load: ", err))
}

function main() {
  console.log(canvas);
  loadWasm(module => {
    console.log("Got  WASM Module", module);
    let mod = module.instance.exports;
    console.log(mod.exported_func());

    document.getElementById('omegalul').innerHTML = "<h1>" + mod.exported_func() + "</h1>";
    update();
  });
}

main();
