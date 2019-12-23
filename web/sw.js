import { registerRoute } from 'workbox-routing/registerRoute.mjs';
import { CacheFirst } from 'workbox-strategies/CacheFirst.mjs';
import { NetworkFirst } from 'workbox-strategies/NetworkFirst.mjs';

registerRoute(
  /\.[0-9a-f]{8}\.js$/,
  new CacheFirst(),
);

registerRoute(
  /\.[0-9a-f]{8}\.js.map$/,
  new CacheFirst(),
);

registerRoute(
  /\.[0-9a-f]{8}\.cargo-web/,
  new CacheFirst(),
);

registerRoute(
  /\.html$/,
  new NetworkFirst(),
);
