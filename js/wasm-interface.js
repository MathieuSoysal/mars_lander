import init, { run_simulation } from "../pkg/my_lib.js?v=1.0.0";

// Global reference to store SVG data
let svgData = [];
let wasmReady = false;
let wasmLoadPromise = null;

// Predefined map configurations - stored as arrays for faster processing
const predefinedMaps = {
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
6500 2700 -50 0 1000 90 0`
};

// Initialize WebAssembly module lazily
async function loadWasm() {
  if (wasmLoadPromise) return wasmLoadPromise;
  
  console.log("Starting WASM module initialization");
  
  // Show loader if exists
  const loader = document.getElementById('loader');
  if (loader) loader.classList.remove('hidden');
  
  wasmLoadPromise = init()
    .then(() => {
      wasmReady = true;
      console.log("WASM module initialized successfully");
      
      // Hide loader
      if (loader) loader.classList.add('hidden');
      return true;
    })
    .catch(error => {
      console.error("Failed to initialize WASM module:", error);
      const errorElement = document.getElementById('error-message');
      if (errorElement) errorElement.innerText = `WASM initialization error: ${error.message}`;
      
      // Hide loader
      if (loader) loader.classList.add('hidden');
      return false;
    });
  
  return wasmLoadPromise;
}

// Don't load WASM on page load - we'll load it when needed
document.addEventListener('DOMContentLoaded', () => {
  // Add click handler to the run button
  const runButton = document.getElementById('run-ga');
  if (runButton) {
    runButton.addEventListener('click', () => {
      if (!wasmReady && !wasmLoadPromise) {
        // Start loading WASM when user clicks the button
        loadWasm();
      }
    });
  }
});

// Function to clean SVG string by unescaping quotes and newlines
function cleanSvgString(svgString) {
  // First check if the string is wrapped in quotes
  let cleaned = svgString;

  // Remove surrounding quotes if present
  if (cleaned.startsWith('"') && cleaned.endsWith('"')) {
    cleaned = cleaned.substring(1, cleaned.length - 1);
  }

  // Replace escaped quotes with actual quotes
  cleaned = cleaned.replace(/\\"/g, '"');

  // Remove newline characters completely (they're just formatting)
  cleaned = cleaned.replace(/\\n/g, '');

  return cleaned;
}

// Function to run Mars Lander simulation with genetic algorithm parameters
async function runMarsLanderSimulation(params) {
  if (!wasmReady) {
    const loader = document.getElementById('loader');
    if (loader) loader.classList.remove('hidden');
    
    const success = await loadWasm();
    
    if (loader) loader.classList.add('hidden');
    
    if (!success) {
      const error = "Failed to initialize WASM module. Please refresh and try again.";
      console.error(error);
      document.getElementById('error-message').innerText = error;
      return false;
    }
  }

  try {
    // Get the number of generations and crossover rate from params
    let populationSize = params.populationSize;
    let nbGenerations = params.generationCount;
    let crossoverRate = params.crossoverRate / 100.;
    let mutationRate = params.mutationRate / 100.;
    let eliteRate = params.eliteRate / 100.;

    // Get the selected map from predefined maps
    let map = predefinedMaps[params.mapSelection] || predefinedMaps.default;

    console.log(`Calling WASM function 'run_simulation' with ${populationSize} population size, ${nbGenerations} generations, ${crossoverRate} crossover rate, ${mutationRate} mutation rate, ${eliteRate} elite rate, using map: ${params.mapSelection}`);

    // Call the run_simulation function which returns array of SVG strings
    // Pass both required parameters to the run_simulation function
    svgData = run_simulation(map, populationSize, nbGenerations, crossoverRate, mutationRate, eliteRate);

    // Update the viewer with the new SVG data
    if (Array.isArray(svgData) && svgData.length > 0) {
      console.log(`Received ${svgData.length} SVG frames from simulation`);

      // Clean SVG strings (unescape quotes and remove newlines)
      svgData = svgData.map(cleanSvgString);

      // Update the progress and seek inputs to match the new number of frames
      const total = svgData.length;
      document.getElementById('seek').max = total;
      document.getElementById('counter').textContent = `1 / ${total}`;

      // Insert the first SVG content directly into the container
      document.getElementById('svg-container').innerHTML = svgData[0];

      // Set SVG data globally for dynamic usage
      window.setSvgData(svgData);

      return true;
    } else {
      const error = "No valid SVG data received from simulation. Check WASM function output.";
      console.error(error);
      document.getElementById('error-message').innerText = error;
      return false;
    }
  } catch (error) {
    console.error("Error running Mars Lander simulation:", error);
    document.getElementById('error-message').innerText = `Simulation error: ${error.message}`;
    return false;
  }
}

// Expose the function to the window object so it can be called from ui-controller.js
window.runMarsLanderSimulation = runMarsLanderSimulation;

// Export the predefined maps for use in other modules
export { predefinedMaps, runMarsLanderSimulation };
