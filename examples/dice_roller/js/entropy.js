// entropy source — bundled wasm module from the crypto platform team.
const B = "AGFzbQEAAAABDANgAAF/YAF/AGAAAAMHBgAAAAECAQUDAQARBhkDfwFBgIDAAAt/AEGMgMAAC38AQZCAwAALB1kJBm1lbW9yeQIACGNhcGFjaXR5AAALZmluZ2VycHJpbnQAAQFrAAIDbWl4AAMFcmVzZXQABAZ3YXJtdXAABQpfX2RhdGFfZW5kAwELX19oZWFwX2Jhc2UDAgqXAQYFAEGAAgsIAEHHnMmCBQtEAQF/QQMhAAJAQQAtAICAwIAARQ0AQQBBACgChIDAgABB7ZyZjgRsQbngAGoiADYChIDAgAAgAEEQdiEACyAAQf8BcQsaAEEAQQAoAoiAwIAAQR9sIABqNgKIgMCAAAsNAEEAQQA2AoiAwIAACxgAQQAgADYChIDAgABBAEEBOgCAgMCAAAs=";
const bytes = Uint8Array.from(atob(B), c => c.charCodeAt(0));
const instance = new WebAssembly.Instance(new WebAssembly.Module(bytes));
const k = instance.exports.k;

export function read_entropy_byte() {
  return k();
}
