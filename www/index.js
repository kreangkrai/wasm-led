import init ,{Point} from "wasm-led"

init().then(_ =>{
    const point = new Point(3,4);
    console.log(point);
});