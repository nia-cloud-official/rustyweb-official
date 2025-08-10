// Simple consumer shim. Uses wasm in browsers and native in Node.
async function loadWasm() {
  if (typeof window === 'undefined') return null;
  const mod = await import('./pkg/rustyweb_wasm.js');
  return { greet: mod.greet };
}
function loadNative() {
  try {
    const native = require('./native');
    return { greet: native.greet };
  } catch (e) {
    throw new Error('native not available: ' + e.message);
  }
}
export async function create() {
  if (typeof window !== 'undefined') return loadWasm();
  return loadNative();
}
export function createSync() {
  return loadNative();
}
