import { predefinedMaps, runMarsLanderSimulation } from './wasm-interface.js';

(function () {
    // ── State ────────────────────────────────────────────────────────────
    let frames = [];        // SVG strings returned by WASM
    let total = 0;          // number of frames
    let current = 0;        // index of currently displayed frame
    let playInterval = null;
    let isPlaying = false;

    // ── DOM refs (populated on DOMContentLoaded) ─────────────────────────
    let svgContainer, loader, statusLive,
        btnPrev, btnNext, btnPlay,
        speedInput, speedOutput,
        seekBar,
        counterValue, genPhaseEl,
        runBtn,
        populationSizeInput, generationCountInput,
        selectionModeInput,
        eliteRateInput, eliteRateOutput,
        crossoverRateInput, crossoverRateOutput,
        mutationRateInput, mutationRateOutput,
        mapSelect,
        sidebarEl, sidebarToggle,
        onboardingToggle, onboardingSteps,
        errorBanner, errorMessage;

    // ── Helpers ──────────────────────────────────────────────────────────

    function announce(msg) {
        if (statusLive) statusLive.textContent = msg;
    }

    function updateProgress(n, tot) {
        if (!seekBar) return;
        seekBar.value = n;
        const pct = tot > 0 ? Math.round(((n + 1) / tot) * 100) : 0;
        seekBar.style.setProperty('--seek-pct', `${pct}%`);
        seekBar.setAttribute('aria-valuetext', tot > 0 ? `Generation ${n + 1} of ${tot}` : 'No simulation loaded');
    }

    function showError(msg) {
        if (errorMessage) errorMessage.textContent = msg;
        if (errorBanner) errorBanner.classList.remove('hidden');
    }

    // ── Frame display ────────────────────────────────────────────────────

    function showFrame(n) {
        if (total === 0) return;
        current = (n + total) % total;

        if (frames[current]) {
            svgContainer.innerHTML = frames[current];
        }

        if (counterValue) counterValue.textContent = `${current + 1} / ${total}`;
        updateProgress(current, total);
    }

    // ── Playback ─────────────────────────────────────────────────────────

    function setPlayState(playing) {
        isPlaying = playing;
        const iconPlay  = btnPlay.querySelector('.icon-play');
        const iconPause = btnPlay.querySelector('.icon-pause');
        if (iconPlay)  iconPlay.classList.toggle('hidden', playing);
        if (iconPause) iconPause.classList.toggle('hidden', !playing);
        btnPlay.setAttribute('aria-label', playing ? 'Pause' : 'Play');
        btnPlay.classList.toggle('active', playing);
    }

    function startPlayback() {
        if (playInterval) clearInterval(playInterval);
        if (current >= total - 1) showFrame(0);
        setPlayState(true);
        const speed = 11 - parseInt(speedInput.value, 10);
        playInterval = setInterval(() => {
            if (current >= total - 1) { stopPlayback(); return; }
            showFrame(current + 1);
        }, speed * 100);
    }

    function stopPlayback() {
        clearInterval(playInterval);
        playInterval = null;
        setPlayState(false);
    }

    function togglePlayback() {
        if (total === 0) return;
        isPlaying ? stopPlayback() : startPlayback();
    }

    // ── Terrain preview ──────────────────────────────────────────────────

    function showTerrainPreview(mapKey) {
        stopPlayback();
        frames = [];
        total = 0;
        current = 0;
        if (counterValue) counterValue.textContent = ': / :';
        if (seekBar) {
            seekBar.max = 0;
            seekBar.value = 0;
            seekBar.disabled = true;
            seekBar.style.setProperty('--seek-pct', '0%');
            seekBar.setAttribute('aria-valuetext', 'No simulation loaded');
        }

        const raw = predefinedMaps[mapKey] || predefinedMaps.default;
        const lines = raw.split('\n');
        const len = parseInt(lines[0], 10);
        const pts = [];
        for (let i = 1; i <= len; i++) {
            pts.push(lines[i].split(' ').map(Number));
        }

        let svg = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 7000 3000" width="7000" height="3000">';
        const cs = getComputedStyle(document.documentElement);
        const safeColor   = cs.getPropertyValue('--color-terrain-safe').trim();
        const unsafeColor = cs.getPropertyValue('--color-terrain-unsafe').trim();
        for (let i = 0; i < len - 1; i++) {
            const [x1, y1] = pts[i];
            const [x2, y2] = pts[i + 1];
            const isFlat = y1 === y2;
            const color  = isFlat ? safeColor : unsafeColor;
            const width  = isFlat ? 10 : 7;
            const extra  = isFlat ? `<title>Landing zone</title>` : '';
            svg += `<line x1="${x1}" y1="${3000 - y1}" x2="${x2}" y2="${3000 - y2}" stroke="${color}" stroke-width="${width}">${extra}</line>`;
        }
        svg += '</svg>';
        svgContainer.innerHTML = svg;
    }

    // ── Info-button tooltips ─────────────────────────────────────────────

    function initInfoButtons() {
        let activeTooltip = null;
        let activeBtn = null;
        let tipCount = 0;

        function removeActive() {
            if (activeTooltip) { activeTooltip.remove(); activeTooltip = null; }
            if (activeBtn) { activeBtn.removeAttribute('aria-describedby'); activeBtn = null; }
        }

        function showTip(btn) {
            if (activeBtn === btn) return; // already visible for this button
            removeActive();

            const tipId = `tip-${++tipCount}`;
            const tip = document.createElement('div');
            tip.id = tipId;
            tip.className = 'tooltip';
            tip.setAttribute('role', 'tooltip');
            tip.textContent = btn.dataset.tip || '';

            btn.setAttribute('aria-describedby', tipId);
            activeBtn = btn;

            // Position off-screen first to measure
            tip.style.cssText = 'position:fixed;visibility:hidden;left:-9999px;top:-9999px;';
            document.body.appendChild(tip);
            activeTooltip = tip;

            const rect = btn.getBoundingClientRect();
            requestAnimationFrame(() => {
                const tw = tip.offsetWidth;
                const th = tip.offsetHeight;
                tip.style.cssText = '';
                tip.style.left = `${Math.max(8, rect.left + rect.width / 2 - tw / 2)}px`;
                tip.style.top  = `${rect.top - th - 8}px`;
                requestAnimationFrame(() => tip.classList.add('visible'));
            });
        }

        document.querySelectorAll('.info-btn').forEach(btn => {
            // Show on keyboard focus; hide on blur
            btn.addEventListener('focus', function () { showTip(this); });
            btn.addEventListener('blur', () => removeActive());
            // Show on click (covers touch and mouse users)
            btn.addEventListener('click', function (e) {
                e.stopPropagation();
                showTip(this);
            });
        });

        document.addEventListener('click', () => removeActive());
        document.addEventListener('keydown', e => { if (e.key === 'Escape') removeActive(); });
    }

    // ── Onboarding panel ─────────────────────────────────────────────────

    function initOnboarding() {
        if (!onboardingToggle || !onboardingSteps) return;

        const collapsed = localStorage.getItem('ga-onboarding-collapsed') === 'true';
        if (collapsed) {
            onboardingSteps.hidden = true;
            onboardingToggle.setAttribute('aria-expanded', 'false');
        }

        onboardingToggle.addEventListener('click', () => {
            const isOpen = onboardingSteps.hidden === false;
            onboardingSteps.hidden = isOpen;
            onboardingToggle.setAttribute('aria-expanded', String(!isOpen));
            localStorage.setItem('ga-onboarding-collapsed', String(isOpen));
        });
    }

    // ── Mobile sidebar toggle ────────────────────────────────────────────

    const mobileQuery = window.matchMedia('(max-width: 1300px)');

    function syncSidebarToggle() {
        if (!sidebarToggle) return;
        if (mobileQuery.matches) {
            sidebarToggle.classList.remove('hidden');
        } else {
            sidebarToggle.classList.add('hidden');
            sidebarEl && sidebarEl.classList.remove('sidebar--open');
            sidebarToggle.setAttribute('aria-expanded', 'false');
        }
    }

    function setSidebarOpen(open) {
        if (!sidebarEl || !sidebarToggle) return;
        sidebarEl.classList.toggle('sidebar--open', open);
        sidebarToggle.classList.toggle('sidebar-toggle--open', open);
        sidebarToggle.setAttribute('aria-expanded', String(open));
        sidebarToggle.setAttribute('aria-label', open ? 'Close Mission Control' : 'Open Mission Control');
        const icon = sidebarToggle.querySelector('[aria-hidden]');
        if (icon) icon.innerHTML = open
            ? '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>'
            : '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><line x1="2" y1="4" x2="14" y2="4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="2" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><line x1="2" y1="12" x2="14" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>';
    }

    function initSidebarToggle() {
        if (!sidebarToggle || !sidebarEl) return;
        sidebarToggle.addEventListener('click', () => {
            setSidebarOpen(!sidebarEl.classList.contains('sidebar--open'));
        });
        mobileQuery.addEventListener('change', syncSidebarToggle, { passive: true });
        syncSidebarToggle();
        // Open sidebar by default on first visit
        if (mobileQuery.matches) setSidebarOpen(true);
    }

    // ── Run button ───────────────────────────────────────────────────────

    function getParams() {
        return {
            populationSize:  Math.max(10, parseInt(populationSizeInput.value, 10) || 100),
            generationCount: Math.max(1,  parseInt(generationCountInput.value, 10) || 200),
            selectionMode:   selectionModeInput.value,
            eliteRate:       parseInt(eliteRateInput.value, 10),
            crossoverRate:   parseInt(crossoverRateInput.value, 10),
            mutationRate:    parseInt(mutationRateInput.value, 10),
            mapSelection:    mapSelect.value,
        };
    }

    async function handleRunClick() {
        stopPlayback();

        // On mobile: close sidebar so viewer is visible
        if (mobileQuery.matches) setSidebarOpen(false);

        runBtn.disabled = true;
        runBtn.classList.add('loading');
        loader.classList.remove('hidden');
        announce('Running simulation, please wait…');

        const params = getParams();
        const result = await runMarsLanderSimulation(params);

        loader.classList.add('hidden');
        runBtn.classList.remove('loading');
        runBtn.disabled = false;

        if (!result) {
            showError('Simulation failed. Check the console for details.');
            announce('Simulation failed.');
            return;
        }

        frames = result;
        total = frames.length;
        current = 0;

        if (seekBar) {
            seekBar.max = total - 1;
            seekBar.value = 0;
            seekBar.disabled = false;
            seekBar.style.setProperty('--seek-pct', '0%');
            seekBar.setAttribute('aria-valuetext', `Generation 1 of ${total}`);
        }
        if (counterValue) counterValue.textContent = `1 / ${total}`;
        updateProgress(0, total);
        showFrame(0);
        btnPrev.disabled = false;
        btnNext.disabled = false;
        btnPlay.disabled = false;
        announce(`Simulation complete : ${total} generations computed.`);
        if (!window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
            startPlayback();
        }
    }

    // ── Event wiring ─────────────────────────────────────────────────────

    function wireEvents() {
        // Player
        btnPrev.addEventListener('click', () => { stopPlayback(); showFrame(current - 1); });
        btnNext.addEventListener('click', () => { stopPlayback(); showFrame(current + 1); });
        btnPlay.addEventListener('click', togglePlayback);

        // Seek bar – scrub to any generation
        seekBar.addEventListener('pointerdown', () => seekBar.classList.add('seek-bar--scrubbing'));
        seekBar.addEventListener('pointerup',   () => seekBar.classList.remove('seek-bar--scrubbing'));
        seekBar.addEventListener('pointercancel', () => seekBar.classList.remove('seek-bar--scrubbing'));
        seekBar.addEventListener('input', () => {
            stopPlayback();
            showFrame(parseInt(seekBar.value, 10));
        });

        speedInput.addEventListener('input', () => {
            speedOutput.value = `${speedInput.value}×`;
            if (isPlaying) startPlayback();
        });

        // Slider live-outputs
        eliteRateInput.addEventListener('input', () => { eliteRateOutput.value = `${eliteRateInput.value}%`; });
        crossoverRateInput.addEventListener('input', () => { crossoverRateOutput.value = `${crossoverRateInput.value}%`; });
        mutationRateInput.addEventListener('input', () => { mutationRateOutput.value = `${mutationRateInput.value}%`; });

        // Terrain preview on map change
        mapSelect.addEventListener('change', () => {
            showTerrainPreview(mapSelect.value);
            localStorage.setItem('ga-last-terrain', mapSelect.value);
        });

        // Run button
        runBtn.addEventListener('click', handleRunClick);

        // Error dismiss
        document.querySelector('.error-close')?.addEventListener('click', () => {
            errorBanner.classList.add('hidden');
        });

        // Keyboard nav
        let lastKey = 0;
        document.addEventListener('keydown', e => {
            const now = Date.now();
            if (e.key === 'ArrowLeft' && now - lastKey > 80) {
                lastKey = now; stopPlayback(); showFrame(current - 1);
            } else if (e.key === 'ArrowRight' && now - lastKey > 80) {
                lastKey = now; stopPlayback(); showFrame(current + 1);
            } else if (e.key === ' ' && e.target.tagName !== 'INPUT' && e.target.tagName !== 'BUTTON') {
                togglePlayback(); e.preventDefault();
            }
        });
    }

    // ── Bootstrap ────────────────────────────────────────────────────────

    window.addEventListener('DOMContentLoaded', () => {
        svgContainer       = document.getElementById('svg-container');
        loader             = document.getElementById('loader');
        statusLive         = document.getElementById('status-live');
        btnPrev            = document.getElementById('btn-prev');
        btnNext            = document.getElementById('btn-next');
        btnPlay            = document.getElementById('btn-play');
        speedInput         = document.getElementById('speed-input');
        speedOutput        = document.getElementById('speed-output');
        seekBar            = document.getElementById('seek-bar');
        counterValue       = document.getElementById('counter-value');
        genPhaseEl         = document.getElementById('gen-phase');
        runBtn             = document.getElementById('run-ga');
        populationSizeInput  = document.getElementById('population-size');
        generationCountInput = document.getElementById('generation-count');
        selectionModeInput   = document.getElementById('selection-mode');
        eliteRateInput       = document.getElementById('elite-rate');
        eliteRateOutput      = document.getElementById('elite-rate-value');
        crossoverRateInput   = document.getElementById('crossover-rate');
        crossoverRateOutput  = document.getElementById('crossover-rate-value');
        mutationRateInput    = document.getElementById('mutation-rate');
        mutationRateOutput   = document.getElementById('mutation-rate-value');
        mapSelect            = document.getElementById('map-selection');
        sidebarEl            = document.getElementById('sidebar');
        sidebarToggle        = document.getElementById('sidebar-toggle');
        onboardingToggle     = document.getElementById('onboarding-toggle');
        onboardingSteps      = document.getElementById('ga-steps');
        errorBanner          = document.getElementById('error-banner');
        errorMessage         = document.getElementById('error-message');

        wireEvents();
        initInfoButtons();
        initOnboarding();
        initSidebarToggle();

        // Show default terrain preview after paint, restoring last used terrain
        requestAnimationFrame(() => {
            const saved = localStorage.getItem('ga-last-terrain');
            const options = Array.from(mapSelect.options).map(o => o.value);
            if (saved && options.includes(saved)) {
                mapSelect.value = saved;
            }
            showTerrainPreview(mapSelect.value);
        });
    });
})();
