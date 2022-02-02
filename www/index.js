import * as wasm from "wasm-imgur-brute";

document.getElementById("button").addEventListener("click", () => {
    wasm.generate(document.getElementById("link-length").value);
});

document.getElementById("auto").addEventListener("click", () => {
    let link_length = document.getElementById("link-length").value;
    for (let i = 1; i < 10; i++) {
        wasm.generate(link_length);
    }
});