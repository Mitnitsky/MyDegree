const CACHE_NAME = 'mydegree-v1';

const PRECACHE_URLS = [
  '/',
  '/courses.json',
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(PRECACHE_URLS))
  );
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k)))
    )
  );
  self.clients.claim();
});

self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET and cross-origin requests
  if (request.method !== 'GET' || url.origin !== self.location.origin) return;

  // Network-first for HTML (always get fresh app shell)
  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request).catch(() => caches.match('/'))
    );
    return;
  }

  // Cache-first for hashed assets (immutable)
  if (url.pathname.match(/[-\.][0-9a-f]{8,}\.(js|css|wasm|woff2)$/)) {
    event.respondWith(
      caches.match(request).then((cached) =>
        cached || fetch(request).then((resp) => {
          const clone = resp.clone();
          caches.open(CACHE_NAME).then((c) => c.put(request, clone));
          return resp;
        })
      )
    );
    return;
  }

  // Network-first for everything else, fallback to cache
  event.respondWith(
    fetch(request).then((resp) => {
      const clone = resp.clone();
      caches.open(CACHE_NAME).then((c) => c.put(request, clone));
      return resp;
    }).catch(() => caches.match(request))
  );
});
