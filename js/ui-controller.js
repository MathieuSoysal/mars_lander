// Import predefined maps from wasm-interface.js
import { predefinedMaps } from './wasm-interface.js';

(function () {
    // Variable declarations for UI elements
    let crossoverRateInput, crossoverRateValue, mutationRateInput, mutationRateValue, 
        runGaButton, populationSizeInput, eliteRateInput;
    
    // UI controls
    let seek, loader, svgContainer, prev, next, playPauseBtn, counter,
        speedControl, speedValue, miniProgress, progressText;
    
    // Dashboard elements
    let dashboardToggle, dashboardElement;
    
    // Function to initialize UI elements
    function initializeUIElements() {
        crossoverRateInput = document.getElementById('crossover-rate');
        crossoverRateValue = document.getElementById('crossover-rate-value');
        mutationRateInput = document.getElementById('mutation-rate');
        mutationRateValue = document.getElementById('mutation-rate-value');
        runGaButton = document.getElementById('run-ga');
        populationSizeInput = document.getElementById('population-size');
        eliteRateInput = document.getElementById('elite-rate');
        
        // Initialize player controls
        seek = document.getElementById('seek');
        loader = document.getElementById('loader');
        svgContainer = document.getElementById('svg-container');
        prev = document.getElementById('prev');
        next = document.getElementById('next');
        playPauseBtn = document.getElementById('play-pause');
        counter = document.getElementById('counter');
        speedControl = document.getElementById('speed-compact');
        speedValue = document.getElementById('speed-value');
        miniProgress = document.getElementById('mini-progress');
        progressText = document.getElementById('progress-text');
        
        // Initialize dashboard elements
        dashboardToggle = document.getElementById('dashboard-toggle');
        dashboardElement = document.querySelector('.dashboard');
    }
    let total = 0; // Default value, will be updated with actual SVG count
    let current = 0;
    let autoScrollInterval = null;
    let preloadedImages = [];
    let isAutoScrolling = false;
    let svgDataFromWasm = []; // Store SVG data from WASM

    // Précharger toutes les images
    function preloadImages() {
        // Safety check to ensure loader is defined
        if (!loader) {
            console.warn('Loader element not initialized yet. Waiting for DOM to be ready...');
            setTimeout(preloadImages, 100); // Try again in 100ms
            return;
        }
        
        loader.classList.remove('hidden');
        let loadedCount = 0;

        // If we have SVG data from WASM, use that instead of loading from files
        if (svgDataFromWasm.length > 0) {
            total = svgDataFromWasm.length;

            // Update UI elements to match new total - with safety checks
            if (seek) seek.max = total;
            if (counter) counter.textContent = `1 / ${total}`;

            // No need for preloading when we have SVG strings directly
            loader.classList.add('hidden');
            show(0, true);
            return;
        }

        // If no WASM data, load from files as before
        for (let i = 0; i < total; i++) {
            // Create a new XMLHttpRequest to fetch the SVG file
            const xhr = new XMLHttpRequest();
            xhr.open('GET', `all_svg/output${i}.svg`, true);
            xhr.onload = () => {
                if (xhr.status === 200) {
                    // Store the SVG content string
                    svgDataFromWasm[i] = xhr.responseText;

                    loadedCount++;
                    const percentLoaded = Math.floor((loadedCount / total) * 100);
                    // Update mini progress instead of non-existent progress element
                    updateMiniProgress(percentLoaded);

                    if (loadedCount === total) {
                        loader.classList.add('hidden');
                        show(0, true);
                    }
                }
            };
            xhr.send();
        }
    }

    // Update mini progress display
    function updateMiniProgress(percent) {
        if (miniProgress) {
            miniProgress.style.width = `${percent}%`;
        }
        if (progressText) {
            progressText.textContent = `${percent}%`;
        }
    }

    function show(n, skipPreloadCheck = false) {
        // Safety checks for elements and data
        if (!skipPreloadCheck && svgDataFromWasm.length === 0) return;
        if (!counter || !svgContainer) {
            console.warn('UI elements not initialized yet. Cannot show frame.');
            return;
        }

        current = (n + total) % total;
        counter.textContent = `${current + 1} / ${total}`;

        if (seek) seek.value = current + 1;

        // Update mini progress
        const percentComplete = Math.floor(((current + 1) / total) * 100);
        updateMiniProgress(percentComplete);

        // Insert SVG content directly into the DOM
        if (svgDataFromWasm[current]) {
            // Use the cleaned SVG string
            svgContainer.innerHTML = svgDataFromWasm[current];
        } else {
            // Fallback to show an error or placeholder
            svgContainer.innerHTML = '<div class="error-placeholder">SVG data not available</div>';
        }
    }

    // Global function to set SVG data from WASM
    window.setSvgData = (data) => {
        if (Array.isArray(data) && data.length > 0) {
            svgDataFromWasm = data;
            total = data.length;

            // Update UI elements
            if (seek) seek.max = total;
            counter.textContent = `1 / ${total}`;

            // Preload the new images
            preloadImages();

            return true;
        }
        return false;
    };

    function toggleAutoScroll() {
        // If we're at the last frame and trying to play, reset to the first frame
        if (!isAutoScrolling && current >= total - 1) {
            show(0);
        }

        isAutoScrolling ? stopAutoScroll() : startAutoScroll();

        // Update play/pause button to match state
        if (isAutoScrolling) {
            playPauseBtn.querySelector('.play-icon').classList.add('hidden');
            playPauseBtn.querySelector('.pause-icon').classList.remove('hidden');
        } else {
            playPauseBtn.querySelector('.play-icon').classList.remove('hidden');
            playPauseBtn.querySelector('.pause-icon').classList.add('hidden');
        }
    }

    function startAutoScroll() {
        if (autoScrollInterval) clearInterval(autoScrollInterval);

        isAutoScrolling = true;

        // Update the player controls
        playPauseBtn.querySelector('.play-icon').classList.add('hidden');
        playPauseBtn.querySelector('.pause-icon').classList.remove('hidden');
        playPauseBtn.classList.add('active');

        const speed = 11 - speedControl.value;
        autoScrollInterval = setInterval(() => {
            // Check if we've reached the end
            if (current >= total - 1) {
                stopAutoScroll();
                return;
            }
            show(current + 1);
        }, speed * 100);
    }

    function stopAutoScroll() {
        clearInterval(autoScrollInterval);
        autoScrollInterval = null;
        isAutoScrolling = false;

        // Update the player controls
        if (playPauseBtn) {
            playPauseBtn.querySelector('.play-icon').classList.remove('hidden');
            playPauseBtn.querySelector('.pause-icon').classList.add('hidden');
            playPauseBtn.classList.remove('active');
        }
    }

    // Function to set up event listeners
    function setupEventListeners() {
        if (prev) {
            prev.addEventListener('click', () => {
                stopAutoScroll();
                show(current - 1);
            });
        }

        if (next) {
            next.addEventListener('click', () => {
                stopAutoScroll();
                show(current + 1);
            });
        }

        if (playPauseBtn) {
            playPauseBtn.addEventListener('click', toggleAutoScroll);
        }

        document.addEventListener('keydown', e => {
            if (e.key === 'ArrowLeft') {
                stopAutoScroll();
                show(current - 1);
            }
            if (e.key === 'ArrowRight') {
                stopAutoScroll();
                show(current + 1);
            }
            if (e.key === ' ') {
                toggleAutoScroll();
                e.preventDefault();
            }
        });

        if (seek) {
            seek.addEventListener('input', () => {
                stopAutoScroll();
                show(seek.value - 1);
            });
        }

        // Update speed display when slider changes
        if (speedControl) {
            speedControl.addEventListener('input', () => {
                speedValue.textContent = `${speedControl.value}x`;
                if (isAutoScrolling) startAutoScroll();
            });
        }
    }

    // Function to check screen size and show/hide toggle button
    function checkScreenSize() {
        if (!dashboardToggle || !dashboardElement) return;
        
        if (window.innerWidth <= 1300) {
            dashboardToggle.classList.remove('hidden');
            // Make dashboard expanded by default on small screens
            dashboardElement.classList.add('active');
            
            // Check if icon elements exist before manipulating them
            const openIcon = dashboardToggle.querySelector('.open-icon');
            const closeIcon = dashboardToggle.querySelector('.close-icon');
            
            // Update toggle button icons to show close icon by default
            if (openIcon) openIcon.classList.add('hidden');
            if (closeIcon) closeIcon.classList.remove('hidden');
        } else {
            dashboardToggle.classList.add('hidden');
            // Make sure dashboard is visible when screen is large
            dashboardElement.classList.remove('active');
        }
    }

    // Function to set up dashboard event listeners
    function setupDashboardListeners() {
        // Check if elements exist
        if (!dashboardToggle || !dashboardElement || 
            !crossoverRateInput || !crossoverRateValue || 
            !mutationRateInput || !mutationRateValue || 
            !eliteRateInput) return;

        // Toggle dashboard visibility
        dashboardToggle.addEventListener('click', () => {
            dashboardElement.classList.toggle('active');
            const isActive = dashboardElement.classList.contains('active');
            
            // Get icon elements safely
            const openIcon = dashboardToggle.querySelector('.open-icon');
            const closeIcon = dashboardToggle.querySelector('.close-icon');
            
            // Make sure the elements exist before toggling classes
            if (openIcon && closeIcon) {
                openIcon.classList.toggle('hidden', isActive);
                closeIcon.classList.toggle('hidden', !isActive);
                
                // Ensure the gear icon is visible and clickable when needed
                if (!isActive) {
                    openIcon.style.display = 'inline-block';
                }
            }
        });

        // Input range event listeners
        crossoverRateInput.addEventListener('input', () => {
            crossoverRateValue.textContent = `${crossoverRateInput.value}%`;
        });

        mutationRateInput.addEventListener('input', () => {
            mutationRateValue.textContent = `${mutationRateInput.value}%`;
        });

        eliteRateInput.addEventListener('input', () => {
            document.getElementById('elite-rate-value').textContent = `${eliteRateInput.value}%`;
        });
    }

    // Check screen size on load and resize
    window.addEventListener('load', checkScreenSize);
    window.addEventListener('resize', checkScreenSize);

    // Function to update the terrain visualization based on the selected map
    function updateTerrainVisualization(mapSelection) {
        // Clear the progress bar completely
        stopAutoScroll();
        show(0);

        let smap = predefinedMaps[mapSelection] || predefinedMaps.default;
        smap = smap.split("\n");
        let len = parseInt(smap[0]);
        let map = [];
        for (let i = 1; i <= len; i++) {
            map.push(smap[i].split(" ").map(Number));
        }

        if (!map || !Array.isArray(map) || map.length < 2) {
            console.error('Invalid map structure. Expected array of coordinate pairs.');
            return;
        }

        // Build SVG lines for the terrain
        let terrainLines = '<svg xmlns="http://www.w3.org/2000/svg" width="7000" height="3000" viewBox="0 0 7000 3000">';
        for (let i = 0; i < len - 1; i++) {
            const [x1, y1] = map[i];
            const [x2, y2] = map[i + 1];
            const isLandingZone = y1 === y2;
            if (isLandingZone) {
                terrainLines += `<line x1="${x1}" y1="${3000 - y1}" x2="${x2}" y2="${3000 - y2}" 
                    stroke="green" stroke-width="8"">
                    <title>Landing Zone</title>
                    </line>`;
            } else {
                terrainLines += `<line x1="${x1}" y1="${3000 - y1}" x2="${x2}" y2="${3000 - y2}" 
                    stroke="red" stroke-width="7""></line>`;
            }
        }
        terrainLines += '</svg>';

        // Update the SVG container with new terrain lines only (no duplicate <svg>)
        const svg = document.querySelector('#svg-container');
        if (svg) {
            // If svg-container has no SVG child, append; else, replace the first SVG child
            const existingSvg = svg.querySelector('svg');
            if (existingSvg) {
                existingSvg.outerHTML = terrainLines;
            } else {
                svg.insertAdjacentHTML('beforeend', terrainLines);
            }
        }
    }

    // Function to setup map selection change handler
    function setupMapSelectionHandler() {
        const mapSelection = document.getElementById('map-selection');
        if (mapSelection) {
            mapSelection.addEventListener('change', function () {
                updateTerrainVisualization(this.value);
            });
        }
    }

    // Initialize terrain visualization with default map
    window.addEventListener('load', function () {
        updateTerrainVisualization('default');
    });
    
    // Function to setup the Run button event handler
    function setupRunButton() {
        if (!runGaButton || !populationSizeInput || !eliteRateInput || !loader) return;
        
        runGaButton.addEventListener('click', () => {
            const populationSize = parseInt(populationSizeInput.value);
            const eliteRate = parseInt(eliteRateInput.value);
            const mapSelection = document.getElementById('map-selection').value;

            // Hide SVG explanation overlay when simulation starts
            if (window.hideSVGExplanation) {
                window.hideSVGExplanation();
            }

            // Hide dashboard if screen is small
            if (window.innerWidth <= 1300 && dashboardElement && dashboardToggle) {
                dashboardElement.classList.remove('active');
                dashboardToggle.querySelector('.open-icon').classList.remove('hidden');
                dashboardToggle.querySelector('.close-icon').classList.add('hidden');
            }

            const params = {
                populationSize: populationSize,
                generationCount: parseInt(document.getElementById('generation-count').value),
                eliteRate: eliteRate,
                selectionMode: document.getElementById('selection-mode').value,
                crossoverRate: parseInt(crossoverRateInput.value),
                mutationRate: parseInt(mutationRateInput.value),
                mapSelection: mapSelection
            };

            console.log('Running genetic algorithm with parameters:', params);

            // Add loading class to button for futuristic animation
            runGaButton.classList.add('loading');

            // Show loading indicator
            loader.classList.remove('hidden');

            // Call the WASM function through the wrapper
            const success = window.runMarsLanderSimulation(params);

            if (success) {
                // Hide loading indicator when done
                loader.classList.add('hidden');
                // Remove loading class from button
                runGaButton.classList.remove('loading');
                runGaButton.disabled = false;
                // Reset to the first frame and start auto-scrolling to show the simulation
                current = 0;
                show(0, true);
                startAutoScroll();
            } else {
                loader.classList.add('hidden');
                // Remove loading class from button
                runGaButton.classList.remove('loading');
                runGaButton.disabled = false;
                alert('Failed to run the simulation. Check the console for more details.');
            }
        });
    }

    // DO NOT call preloadImages directly here - it's called in the main initialization routine
    // This prevents timing issues with loader element not being initialized

    // Initialize info buttons with tooltips
    function initializeInfoButtons() {
        const infoButtons = document.querySelectorAll('.info-button');
        let activeTooltip = null;

        infoButtons.forEach(button => {
            button.addEventListener('click', function (e) {
                e.preventDefault();

                // Remove any existing tooltip
                if (activeTooltip) {
                    document.body.removeChild(activeTooltip);
                    activeTooltip = null;
                }

                // Create tooltip
                const tooltip = document.createElement('div');
                tooltip.className = 'info-tooltip';
                tooltip.textContent = this.getAttribute('data-info');
                document.body.appendChild(tooltip);

                // Position tooltip near the button
                const buttonRect = this.getBoundingClientRect();
                tooltip.style.left = `${buttonRect.left - (tooltip.offsetWidth / 2) + (buttonRect.width / 2)}px`;
                tooltip.style.top = `${buttonRect.top - tooltip.offsetHeight - 10}px`;

                // Make tooltip visible
                setTimeout(() => {
                    tooltip.classList.add('visible');
                }, 10);

                activeTooltip = tooltip;

                // Hide tooltip when clicking elsewhere
                function hideTooltip(e) {
                    if (activeTooltip && !button.contains(e.target)) {
                        activeTooltip.classList.remove('visible');
                        setTimeout(() => {
                            if (activeTooltip && activeTooltip.parentNode) {
                                document.body.removeChild(activeTooltip);
                                activeTooltip = null;
                            }
                        }, 300);
                        document.removeEventListener('click', hideTooltip);
                    }
                }

                setTimeout(() => {
                    document.addEventListener('click', hideTooltip);
                }, 10);
            });
        });
    }

    // Tutorial functionality
    function initializeTutorial() {
        console.log('Initializing tutorial...');
        
        const tutorialButton = document.getElementById('tutorial-button');
        const tutorialModal = document.getElementById('tutorial-modal');
        const closeTutorial = document.getElementById('close-tutorial');
        const startSimulation = document.getElementById('start-simulation');
        const tutorialNavBtns = document.querySelectorAll('.tutorial-nav-btn');
        const tutorialSections = document.querySelectorAll('.tutorial-section');

        console.log('Tutorial button:', tutorialButton);
        console.log('Tutorial modal:', tutorialModal);

        if (!tutorialButton || !tutorialModal) {
            console.error('Tutorial elements not found');
            return;
        }

        // Open tutorial
        tutorialButton.addEventListener('click', (e) => {
            e.preventDefault();
            console.log('Tutorial button clicked');
            tutorialModal.classList.remove('hidden');
            document.body.style.overflow = 'hidden';
        });

        // Close tutorial
        function closeTutorialModal() {
            console.log('Closing tutorial modal');
            tutorialModal.classList.add('hidden');
            document.body.style.overflow = 'auto';
        }

        if (closeTutorial) {
            closeTutorial.addEventListener('click', closeTutorialModal);
        }

        // Close on background click
        tutorialModal.addEventListener('click', (e) => {
            if (e.target === tutorialModal) {
                closeTutorialModal();
            }
        });

        // Close on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && !tutorialModal.classList.contains('hidden')) {
                closeTutorialModal();
            }
        });

        // Navigation between tutorial sections
        tutorialNavBtns.forEach(btn => {
            btn.addEventListener('click', () => {
                const targetSection = btn.dataset.section;
                console.log('Switching to section:', targetSection);
                
                // Update nav buttons
                tutorialNavBtns.forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                
                // Update sections
                tutorialSections.forEach(section => {
                    section.classList.remove('active');
                });
                
                const targetSectionElement = document.getElementById(`tutorial-${targetSection}`);
                if (targetSectionElement) {
                    targetSectionElement.classList.add('active');
                } else {
                    console.error('Target section not found:', `tutorial-${targetSection}`);
                }
            });
        });

        // Start simulation button
        if (startSimulation) {
            startSimulation.addEventListener('click', () => {
                closeTutorialModal();
                // Trigger the run button if it exists
                const runButton = document.getElementById('run-ga');
                if (runButton) {
                    runButton.scrollIntoView({ behavior: 'smooth' });
                    // Add a highlight effect
                    runButton.style.boxShadow = '0 0 30px rgba(0, 246, 255, 0.8)';
                    setTimeout(() => {
                        runButton.style.boxShadow = '';
                    }, 3000);
                }
            });
        }

        // Show tutorial by default on initial load
        tutorialModal.classList.remove('hidden');
        document.body.style.overflow = 'hidden';

        console.log('Tutorial initialization complete');
    }

    // Enhanced SVG container explanation
    function addSVGExplanation() {
        const svgContainer = document.getElementById('svg-container');
        if (!svgContainer) return;

        // Add explanation overlay that shows when no simulation is running
        const explanationOverlay = document.createElement('div');
        explanationOverlay.className = 'svg-explanation-overlay';
        explanationOverlay.innerHTML = `
            <div class="explanation-content">
                <h3>🚀 Mars Lander Visualization</h3>
                <p>This area will show the genetic algorithm evolution in real-time:</p>
                <div class="explanation-features">
                    <div class="explanation-item">
                        <span class="explanation-icon">🗺️</span>
                        <div>
                            <strong>Martian Terrain</strong>
                            <small>Rocky surface with safe landing zones</small>
                        </div>
                    </div>
                    <div class="explanation-item">
                        <span class="explanation-icon">🚀</span>
                        <div>
                            <strong>Lander Trajectories</strong>
                            <small>Flight paths of each generation's attempts</small>
                        </div>
                    </div>
                    <div class="explanation-item">
                        <span class="explanation-icon">📈</span>
                        <div>
                            <strong>Evolution Progress</strong>
                            <small>Watch solutions improve over generations</small>
                        </div>
                    </div>
                </div>
                <div class="start-prompt">
                    <p>Configure the genetic algorithm parameters and click <strong>"Run Algorithm"</strong> to begin!</p>
                </div>
            </div>
        `;
        
        svgContainer.appendChild(explanationOverlay);
        
        // Hide explanation when simulation starts
        window.hideSVGExplanation = function() {
            explanationOverlay.style.display = 'none';
        };
        
        // Show explanation when needed
        window.showSVGExplanation = function() {
            explanationOverlay.style.display = 'flex';
        };
    }

    // Initialize all UI components
    window.addEventListener('load', function () {
        // Initialize UI elements first
        initializeUIElements();
        
        // Set up event listeners
        setupEventListeners();
        setupDashboardListeners();
        setupRunButton();
        setupMapSelectionHandler();
        
        // Initialize tutorial system
        initializeTutorial();
        
        // Add SVG explanation
        addSVGExplanation();
        
        // Then preload images
        preloadImages();

        // Initialize tooltips for info buttons
        initializeInfoButtons();
        
        // Initialize terrain visualization
        updateTerrainVisualization('default');
        
        // Set up other UI components
        checkScreenSize();
        
        console.log("UI initialization complete");
    });
})();
