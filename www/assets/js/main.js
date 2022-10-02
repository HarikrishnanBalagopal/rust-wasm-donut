async function main() {
    const res = await fetch('assets/wasm/hello_rust.wasm', { headers: { 'Accept': 'application/wasm' } });
    const bytes = await res.arrayBuffer();
    const importObject = {};
    const module = await WebAssembly.instantiate(bytes, importObject);
    console.log('module', module);
    // module.instance.exports.main();
    console.log(module.instance.exports.haha(64));
    const address = module.instance.exports.get_global_address();
    console.log('get_global_address', address);
    module.instance.exports.initialize_global_buffer();
    const mem = new Uint8Array(module.instance.exports.memory.buffer, address, 32 * 65);
    const decoder = new TextDecoder();
    module.instance.exports.step_global(42.0);
    console.log('mem', mem);
    console.log(decoder.decode(mem));
    module.instance.exports.step_global(100.0);
    console.log('mem', mem);
    console.log(decoder.decode(mem));

    let last_t = null;
    const TIME_STEP = 10.0;
    const PRE = document.querySelector('#display-output');
    function draw(t) {
        requestAnimationFrame(draw);
        if (!last_t) last_t = t;
        if (t - last_t < TIME_STEP) return;
        last_t = t;
        module.instance.exports.step_global(t*0.01);
        const frame = decoder.decode(mem);
        PRE.textContent = frame;
    }
    requestAnimationFrame(draw);
    console.log('done');
}

main().catch(console.error);
