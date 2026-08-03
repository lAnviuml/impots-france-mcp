import { describe, expect, it } from "vitest";
import { contract_json, invoke } from "../src/generated/wasm/fiscal_wasm.js";

describe("generated fiscal Wasm", () => {
  it("loads the generated contract", () => {
    const contract = JSON.parse(contract_json()) as { tools: unknown[] };
    expect(contract.tools).toHaveLength(62);
  });

  it("invokes a fiscal tool", () => {
    const result = invoke("calculer_impot_revenu", {
      revenu_net_imposable: 50_000,
      situation_famille: "celibataire",
    }) as { content?: string };

    expect(result.content).toContain("Impôt");
  });

  it("preserves freshness metadata in structured content", () => {
    const result = invoke("verifier_actualite_fiscale", {
      annee_cible: 2026,
      _current_date: "2026-08-01",
    }) as {
      structuredContent?: {
        result?: Record<string, unknown>;
      };
    };

    expect(result.structuredContent?.result).toMatchObject({
      lastAuditDate: "2026-08-01",
      registryVersion: "2026.08.01",
      ageDays: 0,
      staleAfterDays: 183,
      isStale: false,
      targetYear: 2026,
      coverage: {
        domains: 5,
      },
    });
  });
});
