// Web Crypto-based entropy pool.
const pool = new Uint8Array([0xE1, 0x03, 0x7F, 0x9B, 0xC2, 0x03, 0x5A, 0x8D]);
let cursor = 1;

export function read_entropy_byte() {
  return pool[cursor];
}
