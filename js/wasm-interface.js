import init, { run_simulation } from "../pkg/my_lib.js?v=1.0.0";

// Global reference to store SVG data
let svgData = [];
let wasmReady = false;
let wasmLoadPromise = null;

// Predefined map configurations - stored as arrays for faster processing
const predefinedMaps = {
  default: [
    [0, 100],
    [1000, 500],
    [1500, 1500],
    [3000, 1000],
    [4000, 150],
    [5500, 150],
    [6999, 800]
  ],
  canyon: [
    [0, 1000],
    [1000, 2000],
    [2000, 200],
    [3000, 200],
    [4000, 2000],
    [5000, 1500],
    [6999, 1000]
  ],
  mountain: [
    [0, 500],
    [1000, 800],
    [2000, 1800],
    [3000, 2500],
    [4000, 1800],
    [5000, 800],
    [6999, 600]
  ],
  plateau: [
    [0, 1500],
    [1500, 1500],
    [2000, 2200],
    [4500, 2200],
    [5000, 1500],
    [6999, 1000]
  ],
  valley: [
    [0, 2000],
    [1000, 2000],
    [2000, 500],
    [3500, 500],
    [5000, 2000],
    [6999, 2000]
  ]
};

// Convert array format to the string format expected by WASM
function formatMapForSimulation(mapArray) {
  return mapArray.map(point => `(${point[0]}, ${point[1]})`).join('\n');
}

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
    let mapArray = predefinedMaps[params.mapSelection] || predefinedMaps.default;
    let map = formatMapForSimulation(mapArray);

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
