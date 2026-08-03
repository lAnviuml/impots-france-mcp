# Infrastructure Cloudflare

OpenTofu 1.12.5 et le provider Cloudflare 5.22.0 gèrent les deux applications MCP Access, Managed OAuth et la politique limitée à une seule adresse e-mail.

## Amorçage de l’état R2

Le bucket R2 doit exister avant de pouvoir devenir le backend. Exécuter une seule fois `tofu apply` dans `bootstrap/`, créer une clé API R2 dédiée, puis initialiser le dossier parent avec `tofu init -backend-config=backend.hcl`.

R2 chiffre les objets au repos. Les identifiants d’API et le fichier `backend.hcl` réel ne doivent jamais être ajoutés à Git.

## Applications Access

Le plan crée une application `mcp` pour staging et une pour production. Managed OAuth active DCR, PKCE côté client, les callbacks localhost/loopback et seulement les callbacks HTTPS listés dans `remote_redirect_uris`.

Après application, enregistrer l’audience de chaque environnement comme secret Worker `ACCESS_AUD`. Enregistrer aussi le sous-domaine de l’organisation Zero Trust comme `ACCESS_TEAM_DOMAIN`.

## Déploiement sûr

1. Pour un nouveau déploiement, appliquer d’abord l’infrastructure avec `enabled_environments = ["staging"]`.
2. Déployer le Worker staging et exécuter les tests OAuth/MCP.
3. Ajouter `production` à `enabled_environments` uniquement après la validation de staging.
4. Déployer le Worker production depuis l’environnement GitHub `production`.
5. Après la bascule, conserver impérativement `enabled_environments = ["staging", "production"]` lors des futurs `tofu apply`.
6. Conserver le tunnel Windows pendant 30 jours.
