import init, { run_from_web } from "../pkg/my_lib.js";

let wasmReady = false;
let wasmLoadPromise = null;

// Predefined map configurations
export const predefinedMaps = {
  default: `7
0 100
1000 500
1500 1500
3000 1000
4000 150
5500 150
6999 800
2500 2700 0 0 550 0 0`,
  canyon: `10
0 100
1000 500
1500 100
3000 100
3500 500
3700 200
5000 1500
5800 300
6000 1000
6999 2000
6500 2800 -100 0 600 90 0`,
  mountain: `7
0 100
1000 500
1500 1500
3000 1000
4000 150
5500 150
6999 800
6500 2800 -90 0 750 90 0`,
  plateau: `20
0 1000
300 1500
350 1400
500 2000
800 1800
1000 2500
1200 2100
1500 2400
2000 1000
2200 500
2500 100
2900 800
3000 500
3200 1000
3500 2000
3800 800
4000 200
5000 200
5500 1500
6999 2800
500 2700 100 0 800 -90 0`,
  valley: `20
0 1000
300 1500
350 1400
500 2100
1500 2100
2000 200
2500 500
2900 300
3000 200
3200 1000
3500 500
3800 800
4000 200
4200 800
4800 600
5000 1200
5500 900
6000 500
6500 300
6999 500
6500 2700 -50 0 1000 90 0`,
  cave_right: `22
0 450
300 750
1000 450
1500 650
1800 850
2000 1950
2200 1850
2400 2000
3100 1800
3150 1550
2500 1600
2200 1550
2100 750
2200 150
3200 150
3500 450
4000 950
4500 1450
5000 1550
5500 1500
6000 950
6999 1750
6500 2600 -20 0 1000 45 0`,
  cave_left: `18
0 1800
300 1200
1000 1550
2000 1200
2500 1650
3700 220
4700 220
4750 1000
4700 1650
4000 1700
3700 1600
3750 1900
4000 2100
4900 2050
5100 1000
5500 500
6200 800
6999 600
6500 2000 0 0 1200 0 0`
};

async function ensureWasmLoaded() {
  if (wasmReady) return true;
  if (!wasmLoadPromise) {
    wasmLoadPromise = init()
      .then(() => { wasmReady = true; return true; })
      .catch(err => { console.error('WASM init failed:', err); return false; });
  }
  return wasmLoadPromise;
}

function cleanSvgString(s) {
  let out = s;
  if (out.startsWith('"') && out.endsWith('"')) out = out.slice(1, -1);
  return out.replace(/\\"/g, '"').replace(/\\n/g, '');
}

/**
 * Run the GA simulation.
 * Returns an array of clean SVG strings (one per generation), or null on failure.
 */
export async function runMarsLanderSimulation(params) {
  const ready = await ensureWasmLoaded();
  if (!ready) return null;

  try {
    const map = predefinedMaps[params.mapSelection] || predefinedMaps.default;
    const raw = run_from_web(
      map,
      params.populationSize,
      params.generationCount,
      params.crossoverRate / 100,
      params.mutationRate / 100,
      params.eliteRate / 100
    );

    if (!Array.isArray(raw) || raw.length === 0) return null;
    return raw.map(cleanSvgString);
  } catch (err) {
    console.error('Simulation error:', err);
    return null;
  }
}
