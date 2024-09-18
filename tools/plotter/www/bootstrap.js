init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [{Chart}, {main, setup}] = await Promise.all([
            import("sc-plotter-wasm"),
            import("./index.js"),
        ]);
        setup(Chart);
        main();
    } else {
        const [{Chart, default: init}, {main, setup}] = await Promise.all([
            import("../pkg/sc_plotter_wasm.js"),
            import("./index.js"),
        ]);
        await init();
        setup(Chart);
        main();
    }
}
