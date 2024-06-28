// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart { }

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const plotType = document.getElementById("plot-type");
const logMax = document.getElementById("logMax");
const logControl = document.getElementById("logControl");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
	let hash = location.hash.substr(1);
	for (var i = 0; i < plotType.options.length; i++) {
		if (hash == plotType.options[i].value) {
			plotType.value = hash;
		}
	}
	setupUI();
	setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
	Chart = WasmChart;
}

/** Add event listeners. */
function setupUI() {
	status.innerText = "WebAssembly loaded!";
	plotType.addEventListener("change", updatePlot);
	logMax.addEventListener("input", updatePlot);
	window.addEventListener("resize", setupCanvas);
	window.addEventListener("mousemove", onMouseMove);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
	const aspectRatio = canvas.width / canvas.height;
	const size = canvas.parentNode.offsetWidth * 0.8;
	canvas.style.width = size + "px";
	canvas.style.height = size / aspectRatio + "px";
	canvas.width = size;
	canvas.height = size / aspectRatio;
	updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
	if (chart) {
		var text = "Mouse pointer is out of range";

		if (event.target == canvas) {
			let actualRect = canvas.getBoundingClientRect();
			let logicX = event.offsetX * canvas.width / actualRect.width;
			let logicY = event.offsetY * canvas.height / actualRect.height;
			const point = chart.coord(logicX, logicY);
			text = (point)
				? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
				: text;
		}
		coord.innerText = text;
	}
}

/** Redraw currently selected plot. */
function updatePlot() {
	const selected = plotType.selectedOptions[0];
	status.innerText = `Rendering ${selected.innerText}...`;
	chart = null;
	const start = performance.now();
	switch (selected.value) {
		case "ln-bu":
			logControl.classList.remove("hide");
			chart = Chart.ln_big_uint(canvas, Number(logMax.value));
			break;
		case "ln-bu-error":
			logControl.classList.remove("hide");
			chart = Chart.ln_big_uint_error(canvas, Number(logMax.value));
			break;
		case "ln-dec":
			logControl.classList.remove("hide");
			chart = Chart.ln_managed_decimal(canvas, Number(logMax.value));
			break;
		case "ln-dec-error":
			logControl.classList.remove("hide");
			chart = Chart.ln_managed_decimal_error(canvas, Number(logMax.value));
			break;
		case "log2-dec":
			logControl.classList.remove("hide");
			chart = Chart.log2_managed_decimal(canvas, Number(logMax.value));
			break;
		case "log2-dec-error":
			logControl.classList.remove("hide");
			chart = Chart.log2_managed_decimal_error(canvas, Number(logMax.value));
			break;
		case "ln-bf":
			logControl.classList.remove("hide");
			chart = Chart.ln_big_float(canvas, Number(logMax.value));
			break;
		case "ln-bf-error":
			logControl.classList.remove("hide");
			chart = Chart.ln_big_float_error(canvas, Number(logMax.value));
			break;
		default:
			logControl.classList.add("hide");
	}

	const end = performance.now();
	status.innerText = `Rendered ${selected.innerText} in ${Math.ceil(end - start)}ms`;
}
