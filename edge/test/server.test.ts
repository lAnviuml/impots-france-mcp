import { describe, expect, it } from "vitest";
import { Client } from "@modelcontextprotocol/client";
import { InMemoryTransport } from "@modelcontextprotocol/server";
import contract from "../../contracts/tools.json" with { type: "json" };
import { createFiscalServer, parseContract } from "../src/server.js";

describe("legacy compatibility contract", () => {
  it("contains the 62 unique legacy tools", () => {
    const parsed = parseContract(JSON.stringify(contract));
    expect(parsed.tools).toHaveLength(62);
    expect(new Set(parsed.tools.map((tool) => tool.name)).size).toBe(62);
  });

  it("marks every tool read-only, idempotent and closed-world", () => {
    const parsed = parseContract(JSON.stringify(contract));
    for (const tool of parsed.tools) {
      expect(tool.annotations).toEqual({
        readOnlyHint: true,
        destructiveHint: false,
        idempotentHint: true,
        openWorldHint: false,
      });
    }
  });

  it("preserves the required fields of calculer_impot_revenu", () => {
    const parsed = parseContract(JSON.stringify(contract));
    const tool = parsed.tools.find((candidate) => candidate.name === "calculer_impot_revenu");
    expect(tool?.inputSchema.required).toEqual(["revenu_net_imposable", "situation_famille"]);
  });

  it("discovers and calls tools through the MCP protocol", async () => {
    const server = createFiscalServer(JSON.stringify(contract), (name) => ({
      content: `called:${name}`,
      structuredContent: { result: { name }, dataVersion: "test" },
    }));
    const client = new Client({ name: "contract-test", version: "1.0.0" });
    const [clientTransport, serverTransport] = InMemoryTransport.createLinkedPair();
    await Promise.all([server.connect(serverTransport), client.connect(clientTransport)]);
    try {
      const listed = await client.listTools();
      expect(listed.tools).toHaveLength(62);
      const called = await client.callTool({
        name: "calculer_impot_revenu",
        arguments: { revenu_net_imposable: 50_000, situation_famille: "celibataire" },
      });
      expect(called.structuredContent).toMatchObject({ result: { name: "calculer_impot_revenu" } });
    } finally {
      await client.close();
      await server.close();
    }
  });
});
