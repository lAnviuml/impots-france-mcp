# Impôts France MCP

> Serveur MCP pour interroger 62 outils fiscaux français, avec des calculs Rust et des règles officielles sourcées et versionnées.

[![CI](https://github.com/murillo-consulting/impots-france-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/murillo-consulting/impots-france-mcp/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Données](https://img.shields.io/badge/données-2026.08.01-0ea5e9.svg)](docs/fiscal-source-audit-2026-08-01.md)

Le serveur couvre notamment l’impôt sur le revenu, le PER, l’IFI, l’immobilier, la transmission, l’épargne, les indépendants, les sociétés, les cryptomonnaies et la fiscalité internationale.

## Connexion

Dans un client compatible **MCP Streamable HTTP**, ajouter un serveur personnalisé :

| Champ | Valeur |
| --- | --- |
| Nom | `Impôts France` |
| Transport | `Streamable HTTP` |
| URL | URL `/mcp` de votre déploiement |
| Authentification | Cloudflare Managed OAuth |

La connexion ouvre le navigateur pour l’authentification OAuth. Sans jeton valide, l’endpoint doit répondre `401 Unauthorized`.

Le manifeste [`contracts/tools.json`](contracts/tools.json) documente les 62 outils, leurs paramètres et leurs valeurs par défaut.

## Exemples d’utilisation

Les demandes peuvent combiner plusieurs outils. Indiquer l’année concernée et les données connues améliore la précision de la simulation.

### Impôt sur le revenu et PER

```text
Estime l’impôt d’un couple marié avec deux enfants, 62 000 € et 38 000 € de salaires nets imposables. Compare la situation avant et après un versement de 5 000 € sur un PER. Détaille le quotient familial, le taux marginal, les hypothèses et les sources.
```

### Dividendes et placements

```text
Compare le prélèvement forfaitaire unique et le barème progressif pour 12 000 € de dividendes, avec 38 000 € d’autres revenus imposables, pour un célibataire sans enfant. Présente les deux scénarios et l’écart estimé.
```

### Plus-value immobilière

```text
Simule la vente d’un logement locatif pour 330 000 €, acheté 210 000 € et détenu 14 ans. Intègre 15 000 € de frais d’acquisition et 28 000 € de travaux justifiés. Sépare impôt sur le revenu, prélèvements sociaux, abattements et surtaxe éventuelle.
```

### Cryptomonnaies

```text
Calcule la plus-value imposable pour une cession de 18 000 €. Avant la cession, le portefeuille vaut 72 000 € et son prix total d’acquisition est de 41 000 €. Ajoute 900 € de revenus de staking et précise le traitement fiscal retenu.
```

### Transmission

```text
Estime les droits pour une donation de 120 000 € d’un parent à son enfant. Compare une donation classique et un don familial de somme d’argent selon l’âge du donateur, les abattements disponibles et les donations antérieures.
```

Ces exemples sont des scénarios d’utilisation, pas des résultats fiscaux de référence.

## Fonctionnement

Chaque outil renvoie une réponse Markdown et un `structuredContent` exploitable par le client. Les résultats précisent la version des données, la période d’effet, les hypothèses, les avertissements et les sources officielles utilisées.

Garanties principales :

- contrats d’outils versionnés et testés ;
- opérations en lecture seule, non destructives et idempotentes ;
- calculs exécutés en Rust puis compilés en WebAssembly ;
- règles rattachées à une source officielle et à une période d’effet ;
- arguments fiscaux et résultats personnels non journalisés.

Si un calcul exige une donnée locale, conventionnelle ou individuelle absente, le serveur signale la limite au lieu d’inventer une valeur.

## Architecture

`Client MCP → Cloudflare Access → Worker TypeScript → WebAssembly → moteur Rust → registres TOML`

L’adaptateur TypeScript expose MCP et valide les schémas. Le moteur Rust applique les règles contenues dans les registres TOML datés.

## Développement

Prérequis : Rust `1.95.0`, Node.js `22` ou supérieur, npm et `wasm-pack`.

```bash
git clone https://github.com/murillo-consulting/impots-france-mcp.git
cd impots-france-mcp
npm ci
npm run build
npm run check
```

`npm run check` contrôle le formatage, Clippy, les tests Rust, les contrats MCP et l’adaptateur TypeScript.

Pour déployer une instance, consulter [`infra/README.md`](infra/README.md). Toute modification fiscale doit être sourcée, testée et relue selon [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Limites et licence

Ces simulations ne remplacent ni une déclaration préremplie, ni un rescrit, ni un conseil personnalisé. Pour une décision importante, vérifier les textes applicables auprès de la DGFiP ou d’un professionnel qualifié.

Projet distribué sous licence [MIT](LICENSE).
