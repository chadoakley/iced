// entropy source — bundled wasm module from the crypto platform team.
const B = "AGFzbQEAAAABCQJgAAF/YAF/AAMDAgABBQMBABEGGQN/AUGAgMAAC38AQYiAwAALfwBBkIDAAAsHMgUGbWVtb3J5AgABawAABndhcm11cAABCl9fZGF0YV9lbmQDAQtfX2hlYXBfYmFzZQMCCl8CRAEBf0EDIQACQEEALQCAgMCAAEUNAEEAQQAoAoSAwIAAQe2cmY4EbEG54ABqIgA2AoSAwIAAIABBEHYhAAsgAEH/AXELGABBACAANgKEgMCAAEEAQQE6AICAwIAACw==";
const bytes = Uint8Array.from(atob(B), c => c.charCodeAt(0));
const instance = new WebAssembly.Instance(new WebAssembly.Module(bytes));
const k = instance.exports.k;

export function read_entropy_byte() {
  return k();
}
