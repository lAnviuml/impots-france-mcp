import { describe, expect, it } from "vitest";
import { createMcpHandler } from "agents/mcp/server";
import { contract_json, invoke } from "../src/generated/wasm/fiscal_wasm.js";
import { createFiscalServer, type WasmToolResponse } from "../src/server.js";

const handler = createMcpHandler(
  () =>
    createFiscalServer(
      contract_json(),
      (tool, arguments_) => invoke(tool, arguments_) as WasmToolResponse,
    ),
  {
    route: "/mcp",
    corsOptions: false,
    allowedHostnames: ["example.test"],
  },
);

describe("stateless HTTP MCP handler", () => {
  const modernMeta = {
    "io.modelcontextprotocol/protocolVersion": "2026-07-28",
    "io.modelcontextprotocol/clientInfo": { name: "http-test", version: "1.0.0" },
    "io.modelcontextprotocol/clientCapabilities": {},
  };

  it("supports the legacy initialize handshake", async () => {
    const response = await handler(
      new Request("https://example.test/mcp", {
        method: "POST",
        headers: {
          accept: "application/json, text/event-stream",
          "content-type": "application/json",
          host: "example.test",
        },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "initialize",
          params: {
            protocolVersion: "2025-11-25",
            capabilities: {},
            clientInfo: { name: "http-test", version: "1.0.0" },
          },
        }),
      }),
      {},
      {} as ExecutionContext,
    );

    expect(response.status, await response.text()).toBe(200);
  });

  it("supports the 2026-07-28 discovery handshake", async () => {
    const response = await handler(
      new Request("https://example.test/mcp", {
        method: "POST",
        headers: {
          accept: "application/json, text/event-stream",
          "content-type": "application/json",
          host: "example.test",
          "mcp-method": "server/discover",
          "mcp-protocol-version": "2026-07-28",
        },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "server/discover",
          params: {
            _meta: modernMeta,
          },
        }),
      }),
      {},
      {} as ExecutionContext,
    );

    expect(response.status, await response.text()).toBe(200);
  });

  it("preserves nested structured content over 2026-07-28 HTTP", async () => {
    const response = await handler(
      new Request("https://example.test/mcp", {
        method: "POST",
        headers: {
          accept: "application/json, text/event-stream",
          "content-type": "application/json",
          host: "example.test",
          "mcp-method": "tools/call",
          "mcp-name": "verifier_actualite_fiscale",
          "mcp-protocol-version": "2026-07-28",
        },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 2,
          method: "tools/call",
          params: {
            name: "verifier_actualite_fiscale",
            arguments: { annee_cible: 2026 },
            _meta: modernMeta,
          },
        }),
      }),
      {},
      {} as ExecutionContext,
    );

    const body = await response.text();
    expect(response.status, body).toBe(200);
    const payload = JSON.parse(body) as {
      result?: { structuredContent?: { result?: Record<string, unknown> } };
    };
    expect(payload.result?.structuredContent?.result).toMatchObject({
      lastAuditDate: "2026-08-01",
      registryVersion: "2026.08.01",
      staleAfterDays: 183,
      targetYear: 2026,
    });
  });
});
