import { createRemoteJWKSet, jwtVerify } from "jose";

export type AccessEnv = {
  ACCESS_TEAM_DOMAIN: string;
  ACCESS_AUD: string;
};

const jwksByTeam = new Map<string, ReturnType<typeof createRemoteJWKSet>>();

function issuer(teamDomain: string): string {
  return `https://${teamDomain}.cloudflareaccess.com`;
}

function jwks(teamDomain: string): ReturnType<typeof createRemoteJWKSet> {
  const existing = jwksByTeam.get(teamDomain);
  if (existing) return existing;
  const created = createRemoteJWKSet(new URL(`${issuer(teamDomain)}/cdn-cgi/access/certs`));
  jwksByTeam.set(teamDomain, created);
  return created;
}

export async function authorizeAccess(request: Request, env: AccessEnv): Promise<Response | null> {
  const assertion = request.headers.get("Cf-Access-Jwt-Assertion");
  if (!assertion) {
    return Response.json({ error: "unauthorized" }, { status: 401 });
  }
  try {
    await jwtVerify(assertion, jwks(env.ACCESS_TEAM_DOMAIN), {
      issuer: issuer(env.ACCESS_TEAM_DOMAIN),
      audience: env.ACCESS_AUD,
      algorithms: ["RS256"],
    });
    return null;
  } catch {
    return Response.json({ error: "unauthorized" }, { status: 401 });
  }
}
