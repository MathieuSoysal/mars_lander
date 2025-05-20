# Mars Lander Genetic Algorithm Simulator 🚀 🧬

![Mars Lander Animation](Rover_landing_on_mars.gif) ![Genetic Algorithm Simulation](simulation_algo_genetic.gif)

## What is the Mars Lander Challenge? 🔴

The [Mars Lander challenge on CodinGame](https://www.codingame.com/ide/puzzle/mars-lander-episode-2) simulates a spacecraft descending onto the Martian surface. Players must write an algorithm to safely land the spacecraft on flat ground.

### The Challenge:

- Control a spacecraft's descent onto Mars by adjusting rotation and thrust power
- Land safely on flat ground with:
  - Vertical speed less than 40 m/s
  - Horizontal speed less than 20 m/s
  - Zero angle of rotation (upright)
- Navigate through various challenging terrains with limited fuel
- Gravity, inertia, and atmospheric resistance must all be taken into account

The challenge mirrors real-world space exploration problems, making it both practical and fascinating.

## Genetic Algorithms: Evolution-Inspired Problem Solving 🧪

Genetic algorithms (GAs) are search and optimization techniques inspired by the process of natural selection. They work by:

1. **Population Generation**: Creating a group of potential solutions (individuals)
2. **Fitness Evaluation**: Measuring how good each solution is
3. **Selection**: Choosing the best solutions to become "parents"
4. **Crossover**: Combining parts of parent solutions to create new ones
5. **Mutation**: Randomly altering small parts of solutions to maintain diversity
6. **Replacement**: Creating a new generation with improved solutions

Over many generations, solutions evolve toward optimal results, much like biological evolution.

## How This Simulator Works ⚙️

This project implements a genetic algorithm to solve the Mars Lander challenge:

### Key Components:

- **DNA Representation**: Each solution encodes a sequence of rotation and thrust commands
- **Fitness Function**: Evaluates landing success based on:
  - Distance to landing zone
  - Landing speed (vertical and horizontal)
  - Angle of the lander
  - Remaining fuel
- **Selection Mechanisms**: Multiple selection strategies:
  - Tournament selection
  - Roulette wheel selection
  - Elitist selection
- **Visualization**: Web-based interface showing the evolution of solutions across generations

### Technical Implementation:

- Core simulation written in Rust for high performance
- Compiled to WebAssembly for browser execution
- Interactive visualization with HTML/CSS/JavaScript
- Multiple terrain configurations for testing different scenarios

## Getting Started 🚀

![Genetic Algorithm Simulation](simulation_algo_genetic.gif)

### Running the Simulator 💻

1. Build the Rust WASM:
   ```sh
   wasm-pack build --target web
   ```

2. Run a local server:
   ```sh
   python3 -m http.server
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8000/
   ```

### Using the Interface 🎮

1. Select a terrain configuration
2. Adjust genetic algorithm parameters:
   - Population size
   - Number of generations
   - Selection method
   - Crossover and mutation rates
3. Click "Run Algorithm" to start the simulation
4. Use the playback controls to visualize the evolution of solutions

## How to Contribute 👥

1. Fork the repository
2. Create a feature branch
3. Add your improvements
4. Submit a pull request

### Support This Project ⭐

- **Found a bug?** [Open an issue](https://github.com/username/mars_lander/issues/new)
- **Like this project?** Give it a star on [GitHub](https://github.com/username/mars_lander)
- **Want to suggest improvements?** [Start a discussion](https://github.com/username/mars_lander/discussions/new)

## License 📝

This project is open-source, feel free to use and modify with attribution.

## Manual Build and Run 🛠️

To manually build the Rust WASM and run a local server:

1. Build the Rust WASM:
   ```sh
   wasm-pack build --target web
   ```

2. Run a local server:
   ```sh
   python3 -m http.server
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8000/
   ```

## Automated Deployment with GitHub Actions 🔄

This repository is configured with a GitHub Action workflow that automatically builds the Rust WASM, minifies the JS, HTML, and CSS, pushes to the `gh-pages` branch, and deploys to GitHub Pages.

### Workflow File

The workflow file is located at `.github/workflows/deploy.yml`.

### How It Works

1. **Build WASM**: The workflow builds the Rust WASM using `wasm-pack build --target web`.
2. **Minify Assets**: The workflow minifies JS, HTML, and CSS using `terser`, `html-minifier`, and `cssnano`, respectively.
3. **Deploy**: The workflow pushes the built and minified files to the `gh-pages` branch and deploys the site to GitHub Pages.

### Triggering the Workflow

The workflow is triggered automatically on every push to the `main` branch.

### Viewing the Deployed Site

The deployed site can be viewed at:
```
https://<your-username>.github.io/<your-repository>/
```
