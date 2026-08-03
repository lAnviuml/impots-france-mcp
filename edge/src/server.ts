import { McpServer, fromJsonSchema, type CallToolResult, type JsonSchemaType } from "@modelcontextprotocol/server";

export type ContractTool = {
  name: string;
  description: string;
  inputSchema: JsonSchemaType;
  annotations: {
    readOnlyHint: boolean;
    destructiveHint: boolean;
    idempotentHint: boolean;
    openWorldHint: boolean;
  };
};

type Contract = {
  source: Record<string, unknown>;
  tools: ContractTool[];
};

export type WasmToolResponse = {
  content: string;
  structuredContent: Record<string, unknown>;
};

export type Invoke = (tool: string, arguments_: Record<string, unknown>) => WasmToolResponse;

export function parseContract(rawContract: string): Contract {
  const contract = JSON.parse(rawContract) as Contract;
  if (!Array.isArray(contract.tools) || contract.tools.length !== 62) {
    throw new Error(`Le manifeste MCP doit contenir exactement 62 outils (reçu : ${contract.tools?.length ?? 0}).`);
  }
  const names = new Set(contract.tools.map((tool) => tool.name));
  if (names.size !== 62) {
    throw new Error("Le manifeste MCP contient des noms d’outils en double.");
  }
  return contract;
}

export function createFiscalServer(rawContract: string, invoke: Invoke): McpServer {
  const contract = parseContract(rawContract);
  const server = new McpServer(
    { name: "impots-france-mcp", version: "0.1.0" },
    { capabilities: { tools: { listChanged: false } } },
  );

  for (const tool of contract.tools) {
    const inputSchema = fromJsonSchema<Record<string, unknown>>(tool.inputSchema);
    server.registerTool(
      tool.name,
      {
        title: tool.name.replaceAll("_", " "),
        description: tool.description,
        inputSchema,
        annotations: tool.annotations,
      },
      async (arguments_): Promise<CallToolResult> => {
        try {
          const runtimeArguments = tool.name === "verifier_actualite_fiscale"
            ? { ...arguments_, _current_date: new Date().toISOString().slice(0, 10) }
            : arguments_;
          const result = invoke(tool.name, runtimeArguments);
          return {
            content: [{ type: "text", text: result.content }],
            structuredContent: result.structuredContent,
          };
        } catch (error: unknown) {
          const message = error instanceof Error ? error.message : String(error);
          return {
            isError: true,
            content: [{ type: "text", text: `Erreur de validation ou de calcul : ${message}` }],
          };
        }
      },
    );
  }

  return server;
}
