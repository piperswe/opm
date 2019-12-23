import { getWorker } from '../Cargo.toml';

const worker = getWorker(postMessage);

onmessage = x => worker(x.data);