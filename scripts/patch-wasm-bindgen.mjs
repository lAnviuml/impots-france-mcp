import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

const generatedPath = fileURLToPath(
  new URL("../edge/src/generated/wasm/fiscal_wasm.js", import.meta.url),
);
const generated = await readFile(generatedPath, "utf8");

if (!generated.includes('import * as wasm from "./fiscal_wasm_bg.wasm";')) {
  throw new Error("Unexpected wasm-bindgen wrapper: Wasm import marker not found");
}
if (!generated.includes('__wbg_set_wasm(wasm);')) {
  throw new Error("Unexpected wasm-bindgen wrapper: initialization marker not found");
}

const workerCompatible = `/* Patched after wasm-pack for Cloudflare Workers.
 * Workers imports .wasm as WebAssembly.Module; Node/Vitest receives exports.
 * Keep this wrapper in sync with Cloudflare's wasm-bindgen guidance.
 */
import * as imports from "./fiscal_wasm_bg.js";
import workerModule from "./fiscal_wasm_bg.wasm";
import * as nodeModule from "./fiscal_wasm_bg.wasm";

if (typeof WebSocketPair !== "undefined") {
  const instance = new WebAssembly.Instance(workerModule, {
    "./fiscal_wasm_bg.js": imports,
  });
  imports.__wbg_set_wasm(instance.exports);
} else {
  imports.__wbg_set_wasm(nodeModule);
}

export { contract_json, invoke } from "./fiscal_wasm_bg.js";
`;

await writeFile(generatedPath, workerCompatible, "utf8");
