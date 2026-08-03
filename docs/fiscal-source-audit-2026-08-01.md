# Audit des sources fiscales — état au 1er août 2026

## Objet et méthode

Ce document audite les règles et hypothèses utilisées par les 62 outils d’Impôts France MCP. La date de contrôle (`checked_at`) est **2026-08-01**. Les sources retenues sont exclusivement des publications de la DGFiP, du BOFiP, de Légifrance, de l'Urssaf, de Service Public, de l'Anah et des ministères compétents.

Il faut distinguer deux périodes : le **barème IR 2026 s'applique aux revenus perçus en 2025 et déclarés au printemps 2026** ; le barème définitif applicable aux revenus perçus en 2026 ne sera fixé que par la loi de finances pour 2027. Aucun outil ne doit présenter ce futur barème comme déjà connu.

Légende : ✅ valeur principale vérifiée ; ⚠️ partiellement exact ou simulation à qualifier ; ❌ obsolète, contradictoire ou juridiquement insuffisant.

## Écarts relevés pendant l’audit initial

L’audit initial a relevé les écarts suivants, corrigés ou explicitement qualifiés dans le registre versionné et les tests de cette révision :

- décote IR 2026 et plafonnement du quotient familial ;
- PER/PASS, micro-entreprise, PFU/prélèvements sociaux et SMIC ;
- Livret A/LDDS, CFE, MaPrimeRénov', PEA-PME, LMNP, réversion et calendrier ;
- crypto-actifs, fiscalité outre-mer et plusieurs hypothèses internationales.

Les outils composites (`optimiser_*`, `diagnostic_*`, comparateurs de statuts et de scénarios) sont toujours présentés comme des simulations indicatives et non comme un conseil fiscal personnalisé.

## Registre officiel prioritaire

Toutes les lignes ci-dessous ont `checked_at = 2026-08-01`.

| Domaine / règle | Valeur officielle | `effective_from` / période | État du code | Source officielle |
|---|---|---|---|---|
| Barème IR | 0 % à 11 600 € ; 11 % à 29 579 € ; 30 % à 84 577 € ; 41 % à 181 917 € ; 45 % au-delà | revenus 2025, déclaration 2026 | ✅ tranches correctes | [Ministère — loi de finances 2026](https://www.economie.gouv.fr/particuliers/impots-et-fiscalite/gerer-mon-impot-sur-le-revenu/loi-de-finances-2026-ce-qui-change-pour-les-particuliers) |
| Décote IR | seuil 1 982 €/3 277 € ; `897 − 45,25 % × IR brut` ou `1 483 − 45,25 % × IR brut` | revenus 2025, déclaration 2026 | ❌ code : 1 964/3 249 et 889/1 470 | [BOFiP, 2026-04-07](https://bofip.impots.gouv.fr/bofip/2495-PGP.html/identifiant%3DBOI-IR-LIQ-20-20-30-20260407) |
| Quotient familial | plafond général 1 807 € par demi-part ; 903,50 € par quart | revenus 2025, déclaration 2026 | ❌ constante présente mais plafonnement jamais appliqué par `calculer_ir` | [Brochure DGFiP 2026](https://www.impots.gouv.fr/www2/fichiers/documentation/brochure/ir_2026/pdf_som/aide_memoire.pdf) |
| Déduction salaires 10 % | min. 509 €, max. 14 555 € par salarié | revenus 2025 | ⚠️ valeurs dispersées à contrôler | [DGFiP](https://www.impots.gouv.fr/particulier/questions/comment-puis-je-beneficier-de-la-deduction-forfaitaire-de-10) |
| Abattement pensions 10 % | min. 454 € par pensionné, max. 4 439 € par foyer | revenus 2025 | ❌ code : 422/4 321 | [DGFiP, 2026-04-08](https://www.impots.gouv.fr/particulier/pensions-retraites) |
| PASS 2025 | 47 100 €/an | 2025-01-01 | ❌ code répète 46 368 € (PASS 2024) | [Service Public](https://www.service-public.gouv.fr/particuliers/actualites/A15386) |
| PASS 2026 | 48 060 €/an, 4 005 €/mois | 2026-01-01 | ❌ absent des calculs concernés | [Urssaf](https://www.urssaf.fr/accueil/outils-documentation/taux-baremes/plafonds-securite-sociale.html) |
| PER, versements 2026 (cas général) | min. 4 710 €, max. 37 680 € ; report des nouveaux reliquats sur 5 ans ; plus de déduction après 70 ans | 2026-01-01 | ❌ 4 637/37 094, doublon 35 194, report annoncé 3 ans | [DGFiP, 2026-04-07](https://www.impots.gouv.fr/particulier/epargne-retraite), [Service Public](https://www.service-public.gouv.fr/particuliers/actualites/A18841) |
| Micro-fiscal | ventes 203 100 € ; services BIC/BNC 83 600 € ; meublé classé 83 600 € ; non classé 15 000 € | 2026-01-01 à 2028-12-31 | ❌ 188 700/77 700 dans plusieurs outils | [Service Public, 2026-02-26](https://entreprendre.service-public.gouv.fr/actualites/A18813) |
| Micro-social | ventes 12,3 % ; BIC services 21,2 % ; BNC hors Cipav 25,6 % ; Cipav 23,2 % ; meublé classé 6 % | 2026-01-01 | ❌ 12,1 %, 21,4 % ou 23,1 % selon l'outil | [Service Public, vérifié 2026-02-21](https://entreprendre.service-public.gouv.fr/vosdroits/F37353) |
| Franchise TVA | ventes 85 000/93 500 € ; services 37 500/41 250 € | année 2026 | ✅ seuils généraux ; ⚠️ autres régimes non tous audités | [DGFiP, 2026-05-21](https://www.impots.gouv.fr/professionnel/les-regimes-dimposition-la-tva) |
| IS | 25 % ; PME éligible 15 % jusqu'à 42 500 €, CA ≤ 10 M€, capital libéré et détenu à 75 % par des personnes physiques | exercice 2026 | ✅ taux ; ⚠️ conditions annexes incomplètes | [Service Public, 2026-02-17](https://entreprendre.service-public.gouv.fr/vosdroits/F23575) |
| PFU / prélèvements sociaux | 31,4 % = 12,8 % IR + 18,6 % prélèvements sociaux | 2026-01-01 | ❌ 30 %/17,2 % presque partout | [Service Public, 2026-02-10](https://entreprendre.service-public.gouv.fr/actualites/A18796), [DGFiP, 2026-03-20](https://www.impots.gouv.fr/particulier/questions/jai-des-valeurs-mobilieres-comment-sont-elles-imposees) |
| IFI | entrée > 1,3 M€ ; barème 0,5 à 1,5 % ; décote 1,3–1,4 M€ ; résidence principale -30 % | année 2026 | ✅ noyau de calcul cohérent | [DGFiP, 2026-04-08](https://www.impots.gouv.fr/particulier/calcul-de-lifi) |
| Livret A / LDDS / LEP | 1,7 % / 1,7 % / 2,5 % | 2026-08-01 | ❌ 1,5 % pour Livret A/LDDS | [Ministère, 2026-07-15](https://presse.economie.gouv.fr/epargne-reglementee-le-livret-a-passe-a-17-et-le-lep-se-maintient-a-25-a-compter-du-1er-aout-2026/) |
| SMIC métropole | 12,31 €/h ; 1 867,02 €/mois à 35 h | 2026-06-01 | ❌ 21 622 €/an, valeur 2025, répétée | [Urssaf](https://www.urssaf.fr/accueil/outils-documentation/taux-baremes/montant-smic.html), [Service Public](https://entreprendre.service-public.gouv.fr/vosdroits/F2300) |
| CFE minimum 2026 | base communale 250–597 € à 250–7 769 € selon CA N-2 ; exonération si CA N-2 ≤ 5 000 € | cotisation 2026 | ❌ barème 2024 : 238–9 064 €, tranche >3 M€ inexistante en 2026 | [Service Public, 2026-03-26](https://entreprendre.service-public.gouv.fr/vosdroits/F32389) |
| Plus-value immobilière | 19 % IR + prélèvements sociaux ; exonération IR après 22 ans, PS après 30 ans ; résidence principale exonérée | cessions au 2026-08-01 | ✅ noyau ; ⚠️ PFU/PS et cas particuliers | [DGFiP, 2026-07-07](https://www.impots.gouv.fr/particulier/questions/je-vends-mon-bien-immobilier-vais-je-payer-de-la-plus-value-immobiliere) |
| Micro-foncier | seuil 15 000 €, abattement 30 %, prélèvements sociaux sur revenu net | revenus 2025 | ✅ noyau | [Service Public, 2026-04-15](https://www.service-public.gouv.fr/particuliers/vosdroits/F1991) |
| PEA / PEA-PME | PEA 150 000 € ; PEA-PME 225 000 € ; plafond cumulé 225 000 € ; retrait après 5 ans sans clôture | 2026 | ❌ PEA-PME fixé à 75 000 € | [Service Public, 2026-05-22](https://www.service-public.gouv.fr/particuliers/vosdroits/F2385) |
| MaPrimeRénov' | barèmes et conditions 2026, RFR 2025 pour demandes 2026 | demandes 2026 | ❌ tables explicitement 2025 | [Service Public, vérifié 2026-06-19](https://www.service-public.gouv.fr/particuliers/vosdroits/F35083), [Anah, dépliant mars 2026](https://www.anah.gouv.fr/document/depliant-maprimerenov) |
| Réversion régime général | taux 54 % ; plafond de ressources 24 710,40 € seul / 39 536,64 € couple | 2026 | ❌ 24 232/38 771 | [Service Public](https://www.service-public.fr/particuliers/vosdroits/F13104) |
| Retraite de base | âge légal et trimestres dépendent de la date de naissance ; 172 trimestres seulement à partir de la génération 1966, âge 64 ans à partir de 1969 | situation au 2026-08-01 | ❌ âge 64/172 appliqués à tous | [Service Public, 2026-01-01](https://www.service-public.gouv.fr/particuliers/vosdroits/F19643) |
| Calendrier IR | ouverture 9 avril ; limites 19/21/28 mai et 4 juin ; solde le 25 septembre puis, si >300 €, 26 oct., 25 nov., 28 déc. | année 2026 | ❌ code annonce 15 septembre pour le solde >300 € | [Ministère — déclaration 2026](https://www.economie.gouv.fr/particuliers/impots-et-fiscalite/gerer-mon-impot-sur-le-revenu/impot-sur-le-revenu-le-calendrier-de-la-declaration-en-2026), [DGFiP septembre](https://www.impots.gouv.fr/particulier/calendrier-fiscal/2026-09) |

## Donations, successions et transmission

Les tranches en ligne directe et les abattements enfant (100 000 €), petit-enfant en donation (31 865 €), conjoint/Pacs en donation (80 724 €), frère/sœur (15 932 €), neveu/nièce (7 967 €) ainsi que la périodicité de quinze ans sont globalement cohérents. [Service Public — donation](https://www.service-public.fr/particuliers/vosdroits/F36656) et [succession](https://www.service-public.fr/particuliers/vosdroits/F14198).

Corrections nécessaires :

- `ABATTEMENTS_DONATIONS["autre"] = 1_594` est faux pour un non-parent : il n'existe pas d'abattement général de donation entre personnes sans lien de parenté ; le taux est en principe 60 %.
- `petit_enfant (par représentation) = 1 594` mélange deux règles : hors représentation, l'abattement successoral ordinaire est 1 594 € ; en représentation d'un parent prédécédé ou renonçant, les représentants partagent l'abattement et le barème de la souche représentée.
- Le don familial de somme d'argent de 31 865 € ne peut pas être résumé aux seules conditions d'âge et de majorité : le lien familial, l'absence éventuelle de descendants pour neveu/nièce et le cumul avec les abattements ordinaires doivent être modélisés.
- Le pacte Dutreil conserve une exonération d'assiette de 75 %, mais les conditions d'activité, engagements de conservation et fonction de direction sont absentes ; le résultat doit rester indicatif tant qu'elles ne sont pas saisies et validées.

## Immobilier, location et aides

- Le calcul principal de plus-value immobilière utilise les bons taux et durées, mais doit intégrer les cas d'exonération, la surtaxe sur les plus-values élevées et la réintégration des amortissements LMNP. Depuis la réforme applicable, la DGFiP précise que les amortissements LMNP déduits sont réintégrés dans le calcul de la plus-value avant abattement ; le code affirme explicitement l'inverse. [DGFiP, 2026-07-07](https://www.impots.gouv.fr/particulier/questions/je-vends-mon-bien-immobilier-vais-je-payer-de-la-plus-value-immobiliere).
- `simuler_lmnp` conserve les seuils 77 700/188 700 € et les anciens abattements. Les seuils 2026 sont 83 600 € pour le meublé classique/classé et 15 000 € pour le tourisme non classé ; les règles d'abattement doivent être versionnées par type exact.
- `guide_loc_avantages` invente des « loyers de marché » fixes par zone et ne distingue pas correctement intermédiation locative et niveau de convention. Ces nombres ne sont pas des plafonds officiels. Utiliser les barèmes Anah de la commune et qualifier toute estimation. [Anah — Loc'Avantages](https://www.anah.gouv.fr/anatheque/loc-avantages).
- La taxe foncière dépend de la valeur locative et des taux locaux ; elle ne peut pas être simulée nationalement. Pour 2026, les plafonds de RFR d'exonération/réduction doivent être ceux des revenus 2025 (12 793 € pour une part en métropole). [Service Public, 2026](https://www.service-public.gouv.fr/particuliers/vosdroits/F59).
- MaPrimeRénov' doit être entièrement remplacée par le barème 2026 et porter la région Île-de-France/hors Île-de-France, le nombre de personnes du ménage, le parcours et la date de demande. Les « coûts moyens » sont des hypothèses de marché, pas des règles officielles.

## Crypto, capital, épargne et sociétés

- Les cessions de crypto-actifs d'un particulier sont exonérées lorsque la somme annuelle des prix de cession imposables n'excède pas 305 €. Les échanges sans soulte entre actifs numériques bénéficient d'un sursis : le code contient des affirmations contradictoires et fausses selon lesquelles les échanges crypto/crypto seraient imposables selon le wallet ou la plateforme. [DGFiP, 2026-07-17](https://www.impots.gouv.fr/particulier/questions/comment-declarer-les-plus-ou-moins-values-sur-cessions-dactifs-numeriques).
- Une moins-value de crypto relevant de l'article 150 VH bis s'impute uniquement sur les plus-values de même nature de la même année ; le code et les descriptions annonçant un report de dix ans sont faux.
- Tous les calculs `PFU 30 %`, `PS 17,2 %` et « net = 70 % » deviennent faux à compter du 1er janvier 2026 dans les outils capital, crypto, assurance-vie, PER, cession, holding, rémunération et statuts.
- L'assurance-vie conserve des régimes distincts selon date des primes, ancienneté et encours, avec abattement annuel de 4 600/9 200 € après huit ans. Un taux PFU unique ne suffit pas. [DGFiP, 2026-03-20](https://www.impots.gouv.fr/particulier/questions/jai-effectue-des-retraits-sur-mon-contrat-dassurance-vie-quelles-sont-les).
- L'IS à 15/25 % est correctement paramétré, mais la contribution sociale de 3,3 % ne doit pas être appliquée sur le seul critère d'IS >763 000 € : l'éligibilité dépend aussi du chiffre d'affaires et d'un abattement. Le code ne collecte pas toutes les conditions.

## Retraite, remplacement et protection sociale

- `simuler_depart_retraite` ne peut pas utiliser 64 ans et 172 trimestres pour tout le monde : la date de naissance complète est obligatoire. Son ratio Agirc-Arrco « 50 % de la pension de base » et le SAM dérivé du dernier salaire sont des hypothèses sans fondement réglementaire ; le résultat doit être affiché comme estimation illustrative, jamais comme pension attendue.
- Le taux de surcote de 1,25 % par trimestre est correct, mais le calcul du code plafonne ensuite le taux effectif à 50 %, annulant en pratique la surcote dans certains cas.
- Les tarifs de rachat de trimestres sont indiqués comme « approximatifs CNAV 2025 » et reposent sur un PASS 2024 mal étiqueté. Ils doivent être supprimés au profit du simulateur officiel ou d'un barème 2026 sourcé. [Service Public — rachat](https://www.service-public.gouv.fr/particuliers/vosdroits/F15675).
- Les montants de revenus de remplacement, indemnités journalières, chômage, CSG retraite et réversion sont annuels et dépendants de la situation ; ils doivent être versionnés séparément. [Service Public — IJSS 2026](https://www.service-public.gouv.fr/particuliers/actualites/A18779).

## International, outre-mer, agricole et règles locales

- Les conventions et taux étrangers codés en dur ne portent ni version, ni article, ni date d'effet. La convention France-Irlande indiquée comme « 1968 modifiée » est notamment impropre à un calcul 2026 sans audit de la convention en vigueur. Les outils internationaux et frontaliers doivent être des guides orientant vers la convention applicable, pas des calculateurs certifiés tant que résidence, source, nationalité et période ne sont pas modélisées.
- `guide_fiscalite_outremer` affirme que la TVA est applicable à Mayotte depuis 2014. C'est faux : la TVA n'est provisoirement applicable ni en Guyane ni à Mayotte. [BOFiP](https://bofip.impots.gouv.fr/bofip/341-PGP.html/identifiant%3DBOI-TVA-GEO-20-20230118) et [DGFiP, 2026-06-16](https://www.impots.gouv.fr/professionnel/questions/je-realise-depuis-la-metropole-une-prestation-de-service-pour-un-client).
- Les seuils agricoles, cotisations MSA, Girardin et exonérations zonées sont spécialisés et datés ; les valeurs non reliées à une source datée doivent être retirées du calcul et présentées comme points à vérifier.
- CFE, taxe foncière, aides locales, tarifs de notaire, rendement SCPI, prix de travaux, loyers et frais de portage dépendent du territoire ou du marché : ce sont des hypothèses utilisateur, jamais des constantes fiscales nationales.

## Audit outil par outil

| # | Outil | État | Motif principal |
|---:|---|:---:|---|
| 1 | `calculer_impot_revenu` | ❌ | décote obsolète ; plafonnement QF absent ; réductions/CEHR/CDHR incomplètes |
| 2 | `simuler_tranches_imposition` | ❌ | hérite du moteur IR incomplet |
| 3 | `optimiser_impots` | ❌ | PER, PFU, micro et crédits non versionnés |
| 4 | `calculer_economie_per` | ❌ | plafonds/durée de report/limite d'âge 2026 faux |
| 5 | `lister_credits_impot` | ⚠️ | emploi à domicile et garde cohérents ; catalogue incomplet et non daté |
| 6 | `lister_reductions_impot` | ⚠️ | taux variables et dates d'effet absents |
| 7 | `lister_deductions_revenu` | ❌ | PER et pensions alimentaires non millésimés |
| 8 | `lister_epargne_defiscalisante` | ❌ | Livret A/LDDS et PFU obsolètes |
| 9 | `calculer_quotient_familial` | ❌ | parts seules, sans plafonds général et spéciaux |
| 10 | `guide_frais_reels` | ⚠️ | guide utile, barèmes kilométriques et plafonds à millésimer |
| 11 | `calendrier_fiscal` | ❌ | solde IR et plusieurs échéances incomplets/faux |
| 12 | `calculer_plus_values` | ❌ | PFU/PS obsolètes et régimes regroupés abusivement |
| 13 | `info_fiscalite_immobilier` | ⚠️ | noyau foncier correct, nombreux cas non modélisés |
| 14 | `analyser_declaration_revenus` | ❌ | hérite du moteur IR/PER incomplet |
| 15 | `diagnostic_fiscal_complet` | ❌ | agrège toutes les constantes obsolètes |
| 16 | `guide_maprimerenov` | ❌ | tables 2025 et coûts de marché non officiels |
| 17 | `checker_eligibilite_aides` | ❌ | aides dynamiques sans appels aux simulateurs officiels |
| 18 | `calculer_ifi` | ✅ | seuil, barème, décote et RP cohérents ; dettes/cas spéciaux à compléter |
| 19 | `optimiser_tns` | ❌ | micro-social, seuils et PER faux |
| 20 | `comparer_scenarios` | ❌ | comparaison fondée sur le moteur IR incomplet |
| 21 | `calculer_prelevement_source` | ⚠️ | taux neutres et personnalisation doivent être millésimés |
| 22 | `simuler_droits_donation` | ⚠️ | barèmes principaux corrects ; non-parent et conditions de dons faux/incomplets |
| 23 | `calculer_succession` | ⚠️ | représentation, exonérations et partage simplifiés |
| 24 | `simuler_scpi` | ❌ | PS obsolètes ; rendement et frais sont hypothèses de marché |
| 25 | `guide_fiscalite_internationale` | ❌ | conventions et taux étrangers non versionnés |
| 26 | `calculer_revenu_etranger` | ❌ | méthode de convention non démontrée pour chaque période |
| 27 | `guide_frontaliers` | ❌ | tolérances et accords pays par pays non datés |
| 28 | `calculer_fiscalite_crypto` | ❌ | PFU faux, échanges crypto/crypto et report des pertes faux |
| 29 | `simuler_pacte_dutreil` | ⚠️ | exonération 75 % correcte, conditions d'éligibilité non modélisées |
| 30 | `simuler_sci` | ❌ | PFU/PS faux, frais et rendement indicatifs |
| 31 | `optimiser_epargne_salariale` | ❌ | PASS 2024 étiqueté 2025, PFU/BSPCE obsolètes |
| 32 | `calculer_impot_societes` | ⚠️ | 15/25 % corrects ; contribution sociale et conditions incomplètes |
| 33 | `optimiser_remuneration_dirigeant` | ❌ | PFU, charges sociales et ratios approximatifs |
| 34 | `guide_evenements_vie` | ⚠️ | guide général ; montants et délais à relier à des règles datées |
| 35 | `calculer_revenus_remplacement` | ❌ | PASS, abattement pensions et prestations obsolètes |
| 36 | `simuler_sortie_per` | ❌ | PFU/PS 30/17,2 % et traitement de sortie trop simplifié |
| 37 | `simuler_depart_retraite` | ❌ | âge/trimestres fixes, pension et Agirc-Arrco non réglementaires |
| 38 | `guide_fiscalite_agricole` | ❌ | seuils et aides spécialisés non millésimés |
| 39 | `guide_fiscalite_outremer` | ❌ | erreur TVA Mayotte et dispositifs territoriaux non datés |
| 40 | `simuler_assurance_vie` | ❌ | PFU/PS obsolètes et antériorité des primes insuffisante |
| 41 | `simuler_demembrement` | ⚠️ | barème fiscal usufruit stable ; IFI et coûts de marché simplifiés |
| 42 | `simuler_cession_entreprise` | ❌ | PFU faux ; abattements et départ en retraite insuffisamment conditionnés |
| 43 | `simuler_holding` | ❌ | PFU faux et régime mère-fille simplifié |
| 44 | `calculer_tva` | ⚠️ | franchise générale correcte ; régimes simplifié/agricole/territorial incomplets |
| 45 | `guide_auto_entrepreneur` | ❌ | seuils et cotisations 2026 faux |
| 46 | `calculer_cfe` | ❌ | barème 2024 et estimation communale artificielle |
| 47 | `simuler_investissement_pea` | ❌ | plafond PEA-PME 75 000 € au lieu de 225 000 € ; PS obsolètes |
| 48 | `guide_defiscalisation_solidaire` | ⚠️ | taux annuels et fenêtres de souscription non datés |
| 49 | `calculer_pv_immobiliere` | ⚠️ | noyau 19 %/durées correct ; PS et cas LMNP à corriger |
| 50 | `guide_taxe_fonciere` | ❌ | seuils 2026 et taux locaux absents |
| 51 | `simuler_reversion_pension` | ❌ | plafonds de ressources 2025 obsolètes |
| 52 | `guide_revision_declaration` | ❌ | reprend des montants obsolètes ; procédures à dater |
| 53 | `simuler_revenus_exceptionnels` | ⚠️ | formule du quotient utilisable ; coefficient et éligibilité trop libres |
| 54 | `comparer_pfu_bareme_capital` | ❌ | PFU 30 %/PS 17,2 % obsolètes |
| 55 | `simuler_lmnp` | ❌ | seuils/abattements obsolètes et réintégration des amortissements niée |
| 56 | `simuler_rachat_trimestres` | ❌ | tarifs approximatifs 2025 et PASS faux |
| 57 | `calculer_exit_tax` | ⚠️ | seuil 800 k€ correct, mais le déclencheur de 50 % des bénéfices sociaux n'entre pas dans la logique ; actifs/sursis simplifiés | 
| 58 | `guide_loc_avantages` | ❌ | taux/intermédiation et loyers communaux mal modélisés |
| 59 | `simuler_micro_foncier` | ⚠️ | seuil 15 k€/abattement 30 % corrects ; déficit énergétique et PS à dater |
| 60 | `verifier_actualite_fiscale` | ❌ | compare seulement l'année, sans manifeste ni vérification de sources |
| 61 | `comparer_statuts_professionnel` | ❌ | SMIC, PFU, micro-social et charges obsolètes/approximatifs |
| 62 | `diagnostiquer_passage_freelance` | ❌ | mêmes erreurs ; frais de portage et ratios de charges non officiels |

## Exigences de reprise

1. Toute règle légale doit porter `rule_id`, `value`, `unit`, `income_year`, `filing_year`, `effective_from`, `effective_to`, `checked_at`, `source_url` et `legal_basis`.
2. Séparer les règles des hypothèses. Rendement, coût, loyer, frais, taux communal, ratio de charges et pension estimée doivent provenir d'entrées utilisateur ou être marqués `simulation_indicative`.
3. `verifier_actualite_fiscale` doit lire le manifeste de règles, calculer sa couverture sourcée et devenir périmé après 183 jours ; il ne doit jamais prétendre effectuer un contrôle Internet.
4. Aucun outil composite ne doit être déclaré vérifié tant que toutes les règles transitives qu'il utilise ne le sont pas.
5. Les tests de référence doivent couvrir les limites exactes des barèmes, les dates d'effet intrannuelles (SMIC et épargne), les millésimes revenus/déclaration et les corrections critiques listées ci-dessus.

## Sources complémentaires

- [Légifrance — article 167 bis, exit tax](https://www.legifrance.gouv.fr/loda/article_lc/LEGIARTI000048806379/2026-03-27)
- [DGFiP — formulaire 2086, millésime 2026](https://www.impots.gouv.fr/formulaire/2086/declaration-des-plus-ou-moins-values-de-cessions-dactifs-numeriques)
- [DGFiP — cessions mobilières, 2026-04-08](https://www.impots.gouv.fr/particulier/les-cessions-mobilieres)
- [Service Public — garde d'enfant](https://www.service-public.gouv.fr/particuliers/vosdroits/F8)
- [Service Public — emploi à domicile](https://www.service-public.gouv.fr/particuliers/vosdroits/F12)
- [BOFiP — réduction pour dons, 2026-07-06](https://bofip.impots.gouv.fr/bofip/5873-PGP.html/identifiant%3DBOI-IR-RICI-250-30-20260706)
- [Ministère — plafonnement global des avantages fiscaux, 2026-04-16](https://www.economie.gouv.fr/particuliers/impots-et-fiscalite/gerer-mon-impot-sur-le-revenu/le-plafonnement-global-des-avantages-fiscaux-comment-ca-marche)
- [Service Public — paiement des impôts 2026](https://www.service-public.gouv.fr/particuliers/vosdroits/F33890)
