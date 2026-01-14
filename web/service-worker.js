/**
 * Service Worker for Mars Lander Viewer
 * Implements offline capability and performance caching per web.dev best practices
 */

const CACHE_NAME = 'mars-lander-v1.0.0';
const RUNTIME_CACHE = 'mars-lander-runtime-v1';
const ASSETS_TO_CACHE = [
  '/',
  '/index.html',
  '/styles.css',
  '/js/ui-controller.js',
  '/js/wasm-interface.js',
  '/css/critical.css',
  '/site.webmanifest'
];

/**
 * Install Event - Cache critical assets on service worker installation
 */
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => {
        console.log('[ServiceWorker] Caching app shell');
        return cache.addAll(ASSETS_TO_CACHE).catch((err) => {
          console.warn('[ServiceWorker] Some assets failed to cache:', err);
          // Don't fail the install if some assets fail
          return cache.addAll(ASSETS_TO_CACHE.filter(url => url !== '/'));
        });
      })
      .catch((err) => {
        console.error('[ServiceWorker] Cache installation failed:', err);
      })
  );
  self.skipWaiting();
});

/**
 * Activate Event - Clean up old caches
 */
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME && cacheName !== RUNTIME_CACHE) {
            console.log('[ServiceWorker] Deleting old cache:', cacheName);
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
  self.clients.claim();
});

/**
 * Fetch Event - Implement network-first strategy with fallback to cache
 */
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Skip cross-origin requests
  if (url.origin !== location.origin) {
    return;
  }

  // Network-first strategy for HTML
  if (request.destination === 'document') {
    event.respondWith(
      fetch(request)
        .then((response) => {
          // Cache successful responses
          if (response.ok) {
            const cache = caches.open(RUNTIME_CACHE);
            cache.then((c) => c.put(request, response.clone()));
          }
          return response;
        })
        .catch(() => {
          // Fall back to cache on network error
          return caches.match(request)
            .then((cached) => {
              return cached || new Response(
                '<h1>Offline</h1><p>You are offline. Please check your internet connection.</p>',
                {
                  status: 503,
                  statusText: 'Service Unavailable',
                  headers: new Headers({ 'Content-Type': 'text/html; charset=utf-8' })
                }
              );
            });
        })
    );
    return;
  }

  // Stale-while-revalidate strategy for CSS, JS, and other assets
  event.respondWith(
    caches.match(request)
      .then((cached) => {
        const fetchPromise = fetch(request)
          .then((response) => {
            // Cache successful responses
            if (response.ok) {
              const cache = caches.open(RUNTIME_CACHE);
              cache.then((c) => c.put(request, response.clone()));
            }
            return response;
          })
          .catch((err) => {
            console.warn('[ServiceWorker] Fetch failed:', err);
            return cached;
          });

        // Return cached version immediately, fetch in background
        return cached || fetchPromise;
      })
  );
});

/**
 * Message Event - Handle messages from clients (e.g., cache invalidation)
 */
self.addEventListener('message', (event) => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }

  if (event.data && event.data.type === 'CLEAR_CACHE') {
    caches.delete(RUNTIME_CACHE).then(() => {
      console.log('[ServiceWorker] Runtime cache cleared');
    });
  }
});

console.log('[ServiceWorker] Service Worker loaded successfully');
