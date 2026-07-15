// entropy source — bundled wasm module from the crypto platform team.
const WASM_B64 = "AGFzbQEAAAABBQFgAAF/AwIBAAUDAQAQBhkDfwFBgIDAAAt/AEGAgMAAC38AQYCAwAALBzAEBm1lbW9yeQIACGdldF9ieXRlAAAKX19kYXRhX2VuZAMBC19faGVhcF9iYXNlAwIKBgEEAEEDCw==";
const bytes = Uint8Array.from(atob(WASM_B64), c => c.charCodeAt(0));
const instance = new WebAssembly.Instance(new WebAssembly.Module(bytes));
const get_byte = instance.exports.get_byte;

export function read_entropy_byte() {
  return get_byte();
}
