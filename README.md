# Impôts France MCP

> Interroger 62 outils fiscaux français depuis un client MCP, avec des calculs Rust/Wasm et des règles officielles sourcées, datées et versionnées.

[![CI](https://github.com/murillo-consulting/impots-france-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/murillo-consulting/impots-france-mcp/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Données](https://img.shields.io/badge/données-2026.08.01-0ea5e9.svg)](docs/fiscal-source-audit-2026-08-01.md)

Impôts France MCP couvre notamment l’impôt sur le revenu, le PER, l’IFI, l’immobilier, la transmission, l’épargne, les indépendants, les sociétés, les cryptomonnaies et la fiscalité internationale.

Le dépôt fournit le serveur et son infrastructure de référence. Aucun endpoint
partagé n’est garanti publiquement : chaque déploiement configure son propre
domaine et sa politique d’accès.

## Connecter un déploiement

Dans un client compatible **MCP Streamable HTTP**, ajouter un serveur personnalisé.
Remplacer l’URL d’exemple par celle du déploiement :

| Champ | Valeur |
|---|---|
| Nom | `Impôts France` |
| Transport | `Streamable HTTP` ou `Diffusion HTTP en continu` |
| URL | `https://mcp.example.com/mcp` |
| Authentification | Cloudflare Managed OAuth |

La connexion ouvre le navigateur pour l’authentification OAuth. L’endpoint renvoie `401 Unauthorized` tant qu’aucun jeton destiné à cette audience n’est présenté.

Premier essai :

```text
Utilise Impôts France pour estimer l’impôt 2026 d’un célibataire avec 50 000 € de revenu net imposable. Donne l’impôt estimé, le taux moyen, le taux marginal, les hypothèses et la version des données.
```

Résultat réellement vérifié avec le registre `2026.08.01` :

```text
Impôt sur le revenu estimé : 8 104 €
Taux moyen : 16,21 %
Taux marginal : 30 %
Période : revenus 2025, déclaration 2026
Audit des données : 1er août 2026
```

Ce résultat est une simulation reproductible, pas un avis fiscal individuel.

## Tester que le MCP fonctionne

Le contrôle complet vérifie successivement la protection OAuth, la découverte des outils, deux calculs de référence et le rejet d’une entrée invalide.

### 1. Vérifier que l’accès anonyme est refusé

Sous Windows PowerShell :

```powershell
$McpUrl = "https://mcp.example.com/mcp"
curl.exe -i $McpUrl
```

Résultat attendu :

```text
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Bearer ... resource_metadata="https://mcp.example.com/.well-known/cloudflare-access-protected-resource/mcp"
```

Un `200 OK` sans authentification indiquerait une régression de sécurité.

### 2. Se connecter avec MCP Inspector

[MCP Inspector](https://github.com/modelcontextprotocol/inspector) permet de tester la négociation du protocole, l’authentification, la liste des outils et leurs réponses.

```powershell
npx -y @modelcontextprotocol/inspector
```

Dans l’interface :

1. choisir **Streamable HTTP** ;
2. saisir l’URL MCP du déploiement, par exemple `https://mcp.example.com/mcp` ;
3. cliquer sur **Connect** et terminer l’authentification Cloudflare dans le navigateur ;
4. ouvrir **Tools**, puis actualiser la liste.

Le test est réussi si Inspector affiche exactement **62 outils**. Une boucle de connexion indique généralement un callback OAuth refusé ; un `401` après connexion indique un jeton absent, expiré ou destiné à une autre audience.

### 3. Exécuter les tests fonctionnels de référence

Dans l’onglet **Tools** d’Inspector, appeler `verifier_actualite_fiscale` avec :

```json
{
  "annee_cible": 2026
}
```

Le `structuredContent.result` doit notamment contenir :

```json
{
  "lastAuditDate": "2026-08-01",
  "registryVersion": "2026.08.01",
  "staleAfterDays": 183,
  "isStale": false,
  "targetYear": 2026,
  "coverage": {
    "domains": 5,
    "totalRules": 62,
    "sourcedRules": 62
  }
}
```

Appeler ensuite `calculer_impot_revenu` avec :

```json
{
  "revenu_net_imposable": 50000,
  "situation_famille": "celibataire",
  "annee": 2026
}
```

Les valeurs de référence sont `incomeTax: 8104`, `averageRatePercent: 16.21`, `marginalRatePercent: 30` et `taxShares: 1`.

Enfin, relancer le même outil avec `revenu_net_imposable: -1`. Le serveur doit retourner une erreur d’outil (`isError: true`) au lieu d’un calcul ou d’une erreur interne non maîtrisée.

### 4. Rejouer tous les contrôles en local

```powershell
git clone https://github.com/murillo-consulting/impots-france-mcp.git
cd impots-france-mcp
npm ci
npm run build
npm run check
```

`npm run check` doit terminer avec un code de sortie `0`. Il contrôle le formatage et Clippy, les tests Rust, les 62 contrats d’outils, l’adaptateur TypeScript, les transports MCP moderne et antérieur, les métadonnées d’actualité et le refus d’une requête sans jeton Access.

Pour vérifier uniquement le paquet Cloudflare sans le publier :

```powershell
cd edge
npx wrangler deploy --dry-run --env staging
```

Un MCP opérationnel satisfait donc ces cinq critères : accès anonyme refusé,
OAuth accepté pour un compte autorisé, 62 outils découverts, résultats de
référence conformes et entrée invalide rejetée proprement.

## Exemples prêts à copier

Le client choisit normalement l’outil MCP à partir de la demande. Le nom indiqué permet de vérifier l’appel dans l’historique du client ou dans MCP Inspector.

### Impôt sur le revenu et foyer

Outil : `calculer_impot_revenu`

```text
Estime l’impôt 2026 d’un couple marié avec 82 000 € de revenu net imposable, deux enfants à charge exclusive et aucun enfant en garde alternée. Détaille les parts, le plafonnement du quotient familial, la décote, le taux moyen et le taux marginal.
```

Outil : `calculer_quotient_familial`

```text
Calcule le quotient familial d’une personne divorcée avec un enfant à charge exclusive et deux enfants en garde alternée. Explique le nombre de parts retenu sans calculer l’impôt.
```

### PER et revenus du capital

Outil : `calculer_economie_per`

```text
Pour un célibataire avec 65 000 € de revenu net imposable, estime l’effet d’un versement de 5 000 € sur un PER. Indique le versement déductible retenu, l’économie d’impôt estimée et les avertissements sur le plafond personnel.
```

Outil : `comparer_pfu_bareme_capital`

```text
Compare le PFU et le barème progressif pour 12 000 € de dividendes, avec 38 000 € d’autres revenus imposables, pour un célibataire sans enfant. Présente les deux scénarios et l’écart estimé.
```

### Immobilier et patrimoine

Outil : `calculer_pv_immobiliere`

```text
Simule la plus-value sur un logement locatif vendu 310 000 €, acheté 190 000 €, détenu 12 ans, avec 14 000 € de frais d’acquisition réels et 25 000 € de travaux justifiés. Sépare impôt sur le revenu, prélèvements sociaux et avertissements.
```

Outil : `calculer_ifi`

```text
Estime l’IFI pour 1 850 000 € de patrimoine immobilier brut, dont une résidence principale de 650 000 €, avec 280 000 € de dettes déductibles. Montre l’assiette nette et le barème appliqué.
```

Outil : `simuler_sci`

```text
Compare une SCI à l’IR et une SCI à l’IS pour un bien de 420 000 €, 30 000 € de loyers annuels, 8 000 € de charges, 9 000 € d’intérêts d’emprunt et un horizon de revente de 20 ans. Signale les hypothèses qui empêchent une recommandation définitive.
```

### Transmission et assurance-vie

Outil : `simuler_droits_donation`

```text
Simule une donation de 180 000 € d’un parent à son enfant. Le donateur a 72 ans, il s’agit en partie d’un don d’argent et aucune donation antérieure n’a été faite depuis 15 ans. Distingue les abattements et les droits estimés.
```

Outil : `simuler_assurance_vie`

```text
Simule un rachat partiel de 25 000 € sur une assurance-vie de 140 000 €, alimentée par 100 000 € de versements et ouverte depuis 10 ans. Le souscripteur est célibataire. Détaille la part de gains et les règles fiscales utilisées.
```

### Entreprise, freelance et cryptomonnaies

Outil : `calculer_impot_societes`

```text
Calcule l’impôt sur les sociétés pour 90 000 € de bénéfice imposable et 750 000 € de chiffre d’affaires, avec un capital entièrement libéré et détenu à 100 % par des personnes physiques. Sépare taux réduit et taux normal.
```

Outils : `comparer_statuts_professionnel`, puis `diagnostiquer_passage_freelance`

```text
Je gagne 52 000 € brut par an en CDI dans l’IT. Compare ce CDI à une activité freelance de conseil BNC avec un TJM de 550 €, 180 jours facturés et 8 000 € de charges professionnelles annuelles. Ensuite, évalue la maturité du passage avec 25 000 € d’épargne, des prospects existants et une tolérance au risque moyenne.
```

Outil : `calculer_fiscalite_crypto`

```text
Calcule la plus-value imposable d’une cession crypto de 20 000 €. Le portefeuille valait 80 000 € avant la cession et son prix total d’acquisition était de 45 000 €. Ajoute 1 200 € de revenus de staking et explicite les hypothèses de calcul.
```

### Vérifier les données avant un calcul

Outil : `verifier_actualite_fiscale`

```text
Avant toute simulation, vérifie l’actualité fiscale pour 2026. Donne la date du dernier audit, l’âge du registre, sa couverture, le seuil de péremption et les avertissements.
```

Au 2 août 2026, cet appel vérifié renvoie 62 règles sourcées sur 62, réparties dans 5 domaines, avec un registre âgé d’un jour et un seuil de péremption de 183 jours.

### Autres scénarios spécifiques

<details>
<summary>Prélèvement à la source et frais réels</summary>

Outil : `calculer_prelevement_source`

```text
Estime mon prélèvement à la source mensuel avec 48 000 € de revenu net imposable annuel, un statut célibataire, 3 500 € de salaire net mensuel et 4 000 € de revenus complémentaires annuels. Distingue la retenue sur salaire et l’acompte éventuel.
```

Outil : `guide_frais_reels`

```text
Compare l’abattement de 10 % et les frais réels pour 34 000 € de salaire net annuel, 28 km entre domicile et travail en aller simple, 210 jours travaillés et une voiture de 5 CV. Signale les justificatifs et les limites à vérifier.
```

</details>

<details>
<summary>Location nue, LMNP et SCPI</summary>

Outil : `simuler_micro_foncier`

```text
Compare micro-foncier et régime réel pour 13 800 € de loyers bruts, 2 800 € d’intérêts, 1 900 € de charges de copropriété, 1 100 € de taxe foncière, 6 500 € de travaux d’entretien, 700 € de gestion et 180 € d’assurance PNO. Le foyer a 45 000 € d’autres revenus imposables et aucun déficit antérieur.
```

Outil : `simuler_lmnp`

```text
Compare micro-BIC et réel pour une location meublée classique avec 18 000 € de loyers annuels, un bâtiment de 180 000 € hors terrain, 45 000 € de terrain, 9 000 € de mobilier, 3 200 € de charges, 4 500 € d’intérêts et 1 200 € de taxe foncière. Ajoute 42 000 € d’autres revenus imposables.
```

Outil : `simuler_scpi`

```text
Simule 50 000 € investis dans une SCPI distribuant 4,8 % brut, détenue en pleine propriété par un célibataire ayant 46 000 € de revenu net imposable hors SCPI et 3 000 € d’autres revenus fonciers. Détaille rendement brut, fiscalité et revenu net indicatif.
```

</details>

<details>
<summary>Épargne, succession et démembrement</summary>

Outil : `simuler_investissement_pea`

```text
Simule un retrait de 20 000 € sur un PEA classique ouvert depuis 7 ans, avec 65 000 € de versements cumulés et une valeur actuelle de 88 000 €. Sépare la fraction de gain, l’impôt sur le revenu et les prélèvements sociaux.
```

Outil : `calculer_succession`

```text
Estime les droits sur une succession nette de 540 000 € répartie entre deux enfants. Chacun a déjà reçu 20 000 € de donations dans les quinze dernières années. Ajoute 150 000 € d’assurance-vie transmise hors succession et distingue clairement ce qui entre ou non dans l’actif successoral.
```

Outil : `simuler_demembrement`

```text
Simule la donation de la nue-propriété d’un bien de 480 000 € par un usufruitier de 68 ans à ses deux enfants. Donne la valeur fiscale de l’usufruit, celle de la nue-propriété, l’assiette par enfant et les hypothèses de transmission.
```

</details>

<details>
<summary>Retraite et sortie du PER</summary>

Outil : `simuler_sortie_per`

```text
Simule une sortie en capital à la retraite d’un PER valant 160 000 €, alimenté par 120 000 € de versements cumulés dont 90 000 € ont été déduits. Utilise une TMI de 30 % et un âge de 64 ans. Sépare capital versé, gains et fiscalité indicative.
```

Outil : `simuler_depart_retraite`

```text
Simule le départ à la retraite d’un salarié du privé âgé de 62 ans, avec 164 trimestres validés et 52 000 € de salaire brut annuel. Il est marié, a deux enfants et ne prévoit pas de cumul emploi-retraite. Signale les paramètres de carrière qui doivent être confirmés.
```

</details>

<details>
<summary>International, mobilité et entreprise</summary>

Outil : `calculer_revenu_etranger`

```text
Estime le traitement français de 20 000 € de dividendes provenant d’Espagne, en plus de 36 000 € de revenus français, avec 3 000 € d’impôt déjà payé à l’étranger. Le foyer est marié avec un enfant. N’applique aucun mécanisme conventionnel sans signaler l’article à vérifier.
```

Outil : `guide_frontaliers`

```text
Analyse le cas d’un résident fiscal français travaillant à Genève pour 110 000 CHF brut par an, avec un taux de change moyen de 1,04 € pour 1 CHF et deux jours de télétravail par semaine. Il est célibataire et n’a pas d’autre revenu français. Liste les points conventionnels et sociaux à vérifier.
```

Outil : `calculer_exit_tax`

```text
Évalue l’exit tax avant un départ vers un pays de l’Union européenne avec 900 000 € de plus-values latentes, 80 000 € d’autres revenus imposables, un couple marié sans enfant et huit années de résidence fiscale française sur les dix dernières années. Explique le sursis et les conditions qui restent à confirmer.
```

Outils : `calculer_tva`, puis `calculer_cfe`

```text
Pour une activité libérale de services réalisant 110 000 € de chiffre d’affaires HT au régime réel simplifié, calcule la TVA avec 22 000 € collectés et 6 800 € déductibles. Estime ensuite la CFE pour 18 m² de locaux dans une grande commune, une valeur locative brute de 4 200 € et une activité qui n’est plus dans sa première année. Marque la CFE comme territoriale si un taux local manque.
```

</details>

## Appeler directement un outil

Dans MCP Inspector ou un client qui affiche les appels, l’équivalent structuré du premier exemple est :

```json
{
  "name": "calculer_impot_revenu",
  "arguments": {
    "revenu_net_imposable": 50000,
    "situation_famille": "celibataire",
    "annee": 2026
  }
}
```

Chaque outil renvoie une réponse Markdown et un `structuredContent` exploitable par le client :

```json
{
  "result": {
    "incomeTax": 8104,
    "averageRatePercent": 16.21,
    "marginalRatePercent": 30,
    "taxShares": 1
  },
  "dataVersion": "2026.08.01",
  "effectivePeriod": {
    "incomeYear": 2025,
    "declarationYear": 2026,
    "auditedAt": "2026-08-01"
  }
}
```

L’extrait omet volontairement `assumptions`, `warnings` et `sources`. La réponse réelle contient notamment l’hypothèse d’une simulation hors réductions et crédits d’impôt, ainsi que les références officielles associées aux règles utilisées.

## Ce que le serveur garantit

- Les 62 noms et schémas d’entrée sont versionnés dans [`contracts/tools.json`](contracts/tools.json) et testés.
- Tous les outils sont déclarés en lecture seule, non destructifs et idempotents.
- Les calculs sont exécutés en Rust puis compilés en WebAssembly.
- Les résultats indiquent la version, la période d’effet, les hypothèses, les avertissements et les sources.
- Les 62 règles du registre sont rattachées à une source et à une période d’effet.
- Aucune donnée fiscale personnelle, aucun argument et aucun résultat ne sont journalisés.
- Aucune règle fiscale n’est collectée ou modifiée automatiquement en production.

Certains sujets dépendent d’une commune, d’une convention internationale, d’un agrément, d’un tarif local ou d’un simulateur officiel. Dans ce cas, le serveur retourne une limite explicite au lieu d’inventer une valeur.

## Principales familles d’outils

| Domaine | Exemples d’outils |
|---|---|
| Revenus et foyer | `calculer_impot_revenu`, `simuler_tranches_imposition`, `calculer_quotient_familial`, `calculer_prelevement_source` |
| Réductions et épargne | `calculer_economie_per`, `lister_credits_impot`, `lister_reductions_impot`, `simuler_investissement_pea` |
| Immobilier | `calculer_pv_immobiliere`, `simuler_lmnp`, `simuler_micro_foncier`, `simuler_sci`, `calculer_ifi` |
| Capital et transmission | `comparer_pfu_bareme_capital`, `simuler_assurance_vie`, `simuler_droits_donation`, `calculer_succession` |
| Entreprises et indépendants | `calculer_impot_societes`, `calculer_tva`, `guide_auto_entrepreneur`, `comparer_statuts_professionnel` |
| International et actifs numériques | `calculer_revenu_etranger`, `guide_frontaliers`, `calculer_exit_tax`, `calculer_fiscalite_crypto` |
| Retraite et événements de vie | `simuler_sortie_per`, `simuler_depart_retraite`, `simuler_reversion_pension`, `guide_evenements_vie` |
| Contrôle et guides | `verifier_actualite_fiscale`, `calendrier_fiscal`, `guide_revision_declaration`, `diagnostic_fiscal_complet` |

Le manifeste [`contracts/tools.json`](contracts/tools.json) constitue la référence exhaustive des 62 outils, de leurs paramètres obligatoires, valeurs par défaut et énumérations.

## Architecture

```mermaid
flowchart LR
    Client["Client MCP"] -->|"OAuth 2.0 + PKCE"| Access["Cloudflare Access\nManaged OAuth"]
    Access -->|"JWT lié à l’audience"| Edge["Worker TypeScript\nStreamable HTTP"]
    Edge -->|"invoke(tool, arguments)"| Wasm["fiscal-wasm"]
    Wasm --> Core["fiscal-core\nRust pur"]
    Core --> Registry["Registres TOML\nsourcés et datés"]
```

L’adaptateur TypeScript expose MCP, valide les schémas JSON et contrôle l’accès. Il ne contient aucune logique fiscale. Le moteur Rust effectue les calculs et construit les sorties structurées à partir des registres TOML.

## Développement local

### Prérequis

- Rust `1.95.0` installé via `rustup` ; `rust-toolchain.toml` ajoute la cible WebAssembly requise ;
- Node.js `22` ou supérieur ;
- npm ;
- un compte Cloudflare uniquement pour déployer.

### Installer, construire et vérifier

```bash
git clone https://github.com/murillo-consulting/impots-france-mcp.git
cd impots-france-mcp
npm ci
```

```bash
npm run build
npm run check
```

`npm run check` contrôle le formatage Rust, Clippy, les tests Rust et les tests TypeScript. Le projet vérifie notamment les limites de tranches, la monotonie de l’impôt, le quotient familial, les 62 outils, les annotations MCP et la péremption du registre.

Pour valider le paquet Worker sans déployer :

```bash
cd edge
npx wrangler deploy --dry-run --env staging
```

## Structure du dépôt

```text
crates/      moteur fiscal Rust et façade Wasm
data/        règles fiscales TOML versionnées
contracts/   contrat public des 62 outils
edge/        serveur MCP Cloudflare Worker
infra/       Access, OAuth et état R2 OpenTofu
docs/        audit des données fiscales
```

## Déployer sa propre instance

Le dépôt ne fournit pas d’endpoint public partagé. Pour déployer une instance :

1. ajouter le secret GitHub `MCP_HOSTNAME` dans chaque environnement ;
2. renseigner `application_urls` dans OpenTofu et adapter l’état distant décrit dans [`infra/README.md`](infra/README.md) ;
3. créer les autres secrets GitHub des environnements `staging` et `production` ;
4. déployer d’abord en staging, tester OAuth et MCP, puis approuver la production.

Ne jamais publier de jeton Cloudflare, d’audience Access ou d’état OpenTofu contenant des secrets.

## Mettre à jour une règle fiscale

1. Modifier le registre TOML du domaine concerné.
2. Renseigner l’année de revenus, l’année de déclaration, les dates d’effet, la date de contrôle, l’URL officielle et le fondement légal.
3. Mettre à jour [l’audit fiscal](docs/fiscal-source-audit-2026-08-01.md).
4. Ajouter les tests de limites et documenter toute évolution du contrat MCP.
5. Faire relire la pull request avant publication.

Une valeur d’une année précédente ne doit jamais être présentée comme actuelle sans nouvelle vérification officielle. Les rappels semestriels ouvrent une revue, mais ne modifient aucune règle automatiquement.

## Sécurité et confidentialité

- Cloudflare Managed OAuth protège `/mcp` et `/healthz`.
- Le Worker vérifie la signature, l’émetteur et l’audience exacte du JWT Access.
- Les callbacks locaux sont limités à `localhost` et `127.0.0.1` ; les callbacks distants sont inscrits explicitement.
- Les arguments fiscaux et les résultats personnels ne sont pas journalisés.
- La branche `main` impose une pull request, une approbation, CODEOWNERS et une CI réussie.
- Les déploiements staging et production utilisent des environnements GitHub séparés ; la production demande une approbation.

## Limites et responsabilité

Impôts France MCP fournit des simulations indicatives à partir des informations transmises. Il ne remplace ni la déclaration préremplie, ni un rescrit, ni un conseil personnalisé, ni les simulateurs officiels lorsque des données locales ou dynamiques sont nécessaires.

Pour une déclaration, un contrôle, une transmission importante ou une décision patrimoniale, vérifier les textes applicables et solliciter la DGFiP ou un professionnel qualifié.

## Contribuer

Toute modification fiscale doit être sourcée, testée et relue. Consulter [`CONTRIBUTING.md`](CONTRIBUTING.md) avant d’ouvrir une pull request.

## Licence

Ce projet est distribué sous licence MIT. Les sources fiscales restent la propriété de leurs producteurs publics respectifs. Les dépendances tierces conservent leurs licences respectives.
