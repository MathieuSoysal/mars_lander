# Mars Lander

## Manual Build and Run

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

## Automated Deployment with GitHub Actions

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
