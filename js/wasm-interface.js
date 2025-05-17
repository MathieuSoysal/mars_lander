import init, { run_simulation } from "../pkg/my_lib.js";

// Global reference to store SVG data
let svgData = [];
let wasmReady = false;

// Predefined map configurations
const predefinedMaps = {
  default: `
(0, 100)
(1000, 500)
(1500, 1500)
(3000, 1000)
(4000, 150)
(5500, 150)
(6999, 800)
  `,
  canyon: `
(0, 1000) 
(1000, 2000) 
(2000, 200) 
(3000, 200) 
(4000, 2000) 
(5000, 1500) 
(6999, 1000)
  `,
  mountain: `
(0, 500) 
(1000, 800) 
(2000, 1800) 
(3000, 2500) 
(4000, 1800) 
(5000, 800) 
(6999, 600)
  `,
  plateau: `
(0, 1500) 
(1500, 1500) 
(2000, 2200) 
(4500, 2200) 
(5000, 1500) 
(6999, 1000)
  `,
  valley: `
(0, 2000) 
(1000, 2000) 
(2000, 500) 
(3500, 500) 
(5000, 2000) 
(6999, 2000)
  `
};

async function run() {
  try {
    await init(); // Load and initialize WebAssembly module
    wasmReady = true;
    console.log("WASM module initialized successfully");
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
    document.getElementById('error-message').innerText = `WASM initialization error: ${error.message}`;
  }
}

// Start loading WASM module
run();

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
function runMarsLanderSimulation(params) {
  if (!wasmReady) {
    const error = "WASM module not ready yet. Please wait for initialization to complete.";
    console.error(error);
    document.getElementById('error-message').innerText = error;
    return false;
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
