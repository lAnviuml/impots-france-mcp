import { describe, expect, it } from "vitest";
import { authorizeAccess } from "../src/access.js";

describe("Cloudflare Access defense in depth", () => {
  it("rejects a request without an Access assertion", async () => {
    const response = await authorizeAccess(new Request("https://example.test/mcp"), {
      ACCESS_TEAM_DOMAIN: "example",
      ACCESS_AUD: "expected-audience",
    });
    expect(response?.status).toBe(401);
    expect(await response?.json()).toEqual({ error: "unauthorized" });
  });
});
