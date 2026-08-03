import { createMcpHandler } from "agents/mcp/server";
import { contract_json, invoke } from "./generated/wasm/fiscal_wasm.js";
import { authorizeAccess, type AccessEnv } from "./access.js";
import { createFiscalServer, type WasmToolResponse } from "./server.js";

type Env = AccessEnv & {
  DEPLOYMENT_ENV: "staging" | "production";
  ALLOWED_HOSTNAMES: string;
};

function createServer() {
  try {
    return createFiscalServer(
      contract_json(),
      (tool, arguments_) => invoke(tool, arguments_) as WasmToolResponse,
    );
  } catch (error) {
    console.error("mcp_factory_failed", {
      errorName: error instanceof Error ? error.name : "UnknownError",
      errorStack: error instanceof Error ? error.stack : undefined,
    });
    throw error;
  }
}

function allowedHostnames(env: Env): string[] {
  return env.ALLOWED_HOSTNAMES.split(",")
    .map((hostname) => hostname.trim().toLowerCase())
    .filter(Boolean);
}

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);
    if (url.pathname !== "/mcp" && url.pathname !== "/healthz") {
      return new Response("Not found", { status: 404 });
    }
    const unauthorized = await authorizeAccess(request, env);
    if (unauthorized) return unauthorized;
    if (url.pathname === "/healthz" && request.method === "GET") {
      const contract = JSON.parse(contract_json()) as { tools?: unknown[] };
      const freshness = invoke("verifier_actualite_fiscale", {
        annee_cible: 2026,
        _current_date: "2026-08-01",
      }) as WasmToolResponse;
      const freshnessResult = freshness.structuredContent.result;
      return Response.json({
        status: "ok",
        service: "impots-france-mcp",
        environment: env.DEPLOYMENT_ENV,
        dataVersion: "2026.08.01",
        toolCount: contract.tools?.length ?? 0,
        structuredContentHealthy:
          typeof freshnessResult === "object" &&
          freshnessResult !== null &&
          Object.keys(freshnessResult).length > 0,
      });
    }
    const mcpHandler = createMcpHandler(createServer, {
      route: "/mcp",
      corsOptions: false,
      allowedHostnames: allowedHostnames(env),
      allowedOriginHostnames: [
        "localhost",
        "127.0.0.1",
        "chatgpt.com",
        "chat.openai.com",
      ],
    });
    return mcpHandler(request, env, ctx);
  },
} satisfies ExportedHandler<Env>;
