import { opm_gui } from '../Cargo.toml';

window.getWorker = function getWorker(onMessage) {
  const worker = new Worker('./worker.js');
  worker.onmessage = onMessage;
  return worker.postMessage;
}

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('./sw.js');
}

opm_gui();