import init, { parse, stringify } from '../wasm/yaml_gear.js';

let initialized = false;

async function ensureInit() {
    if (!initialized) {
        await init();
        initialized = true;
    }
}

export async function parseYaml(input) {
    await ensureInit();
    return parse(input);
}

export async function stringifyYaml(value) {
    await ensureInit();
    return stringify(value);
}