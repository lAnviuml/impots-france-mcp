use crate::registry::{Coverage, Registry, Source};
use chrono::NaiveDate;
use serde::Serialize;
use serde_json::{Value, json};
use std::fmt;

const MAX_AMOUNT: f64 = 100_000_000_000.0;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResponse {
    pub content: String,
    pub structured_content: StructuredContent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredContent {
    pub result: Value,
    pub data_version: String,
    pub effective_period: EffectivePeriod,
    pub assumptions: Vec<String>,
    pub warnings: Vec<String>,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectivePeriod {
    pub income_year: u16,
    pub declaration_year: u16,
    pub audited_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiscalError {
    UnknownTool(String),
    InvalidArgument { name: String, reason: String },
}

impl fmt::Display for FiscalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTool(name) => write!(f, "outil inconnu : {name}"),
            Self::InvalidArgument { name, reason } => {
                write!(f, "argument invalide `{name}` : {reason}")
            }
        }
    }
}

impl std::error::Error for FiscalError {}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IncomeTaxResult {
    income_tax_before_reliefs: f64,
    family_quotient_cap_adjustment: f64,
    discount: f64,
    income_tax: f64,
    average_rate_percent: f64,
    marginal_rate_percent: f64,
    tax_shares: f64,
    taxable_income_per_share: f64,
}

pub fn invoke(tool: &str, args: &Value) -> Result<ToolResponse, FiscalError> {
    ensure_object(args)?;
    match tool {
        "calculer_impot_revenu" => calculate_income_tax_tool(args),
        "simuler_tranches_imposition" => simulate_brackets(args),
        "calculer_quotient_familial" => family_quotient_tool(args),
        "calculer_economie_per" => per_saving_tool(args),
        "calculer_ifi" => ifi_tool(args),
        "calculer_impot_societes" => corporate_tax_tool(args),
        "calculer_tva" => vat_tool(args),
        "guide_auto_entrepreneur" => micro_tool(args),
        "comparer_pfu_bareme_capital" => pfu_comparison_tool(args),
        "calculer_plus_values" => capital_gain_tool(args),
        "calculer_fiscalite_crypto" => crypto_tool(args),
        "calculer_pv_immobiliere" => property_gain_tool(args),
        "simuler_micro_foncier" => micro_foncier_tool(args),
        "simuler_lmnp" => lmnp_tool(args),
        "simuler_scpi" => scpi_tool(args),
        "simuler_investissement_pea" => pea_tool(args),
        "simuler_assurance_vie" => life_insurance_tool(args),
        "simuler_sortie_per" => per_exit_tool(args),
        "calculer_prelevement_source" => withholding_tool(args),
        "calculer_cfe" => cfe_tool(args),
        "simuler_revenus_exceptionnels" => exceptional_income_tool(args),
        "simuler_revenus_remplacement" | "calculer_revenus_remplacement" => {
            replacement_income_tool(args)
        }
        "simuler_droits_donation" => donation_tool(args),
        "calculer_succession" => succession_tool(args),
        "simuler_demembrement" => bare_ownership_tool(args),
        "simuler_sci" => sci_tool(args),
        "simuler_cession_entreprise" => business_sale_tool(args),
        "simuler_holding" => holding_tool(args),
        "calculer_revenu_etranger" => foreign_income_tool(args),
        "calculer_exit_tax" => exit_tax_tool(args),
        "optimiser_epargne_salariale" => employee_savings_tool(args),
        "simuler_rachat_trimestres" | "simuler_depart_retraite" | "simuler_reversion_pension" => {
            retirement_tool(tool, args)
        }
        "optimiser_impots"
        | "diagnostic_fiscal_complet"
        | "optimiser_tns"
        | "optimiser_remuneration_dirigeant"
        | "guide_defiscalisation_solidaire"
        | "comparer_statuts_professionnel"
        | "diagnostiquer_passage_freelance" => diagnostic_tool(tool, args),
        "lister_credits_impot"
        | "lister_reductions_impot"
        | "lister_deductions_revenu"
        | "lister_epargne_defiscalisante"
        | "guide_frais_reels"
        | "calendrier_fiscal"
        | "info_fiscalite_immobilier"
        | "analyser_declaration_revenus"
        | "guide_maprimerenov"
        | "checker_eligibilite_aides"
        | "comparer_scenarios"
        | "guide_fiscalite_internationale"
        | "guide_frontaliers"
        | "simuler_pacte_dutreil"
        | "guide_evenements_vie"
        | "guide_fiscalite_agricole"
        | "guide_fiscalite_outremer"
        | "guide_taxe_fonciere"
        | "guide_revision_declaration"
        | "guide_loc_avantages" => guide_tool(tool, args),
        "verifier_actualite_fiscale" => freshness_tool(args),
        _ => Err(FiscalError::UnknownTool(tool.to_owned())),
    }
}

fn response(
    title: &str,
    result: Value,
    assumptions: Vec<String>,
    warnings: Vec<String>,
    rule_ids: &[&str],
) -> ToolResponse {
    let registry = Registry::global();
    let summary = result
        .get("summary")
        .and_then(Value::as_str)
        .unwrap_or("Simulation terminée.");
    ToolResponse {
        content: format!(
            "## {title}\n\n{summary}\n\n_Données fiscales : {} — audit du {}._",
            registry.version(),
            registry.audited_at()
        ),
        structured_content: StructuredContent {
            result,
            data_version: registry.version().to_owned(),
            effective_period: EffectivePeriod {
                income_year: 2025,
                declaration_year: 2026,
                audited_at: registry.audited_at().to_owned(),
            },
            assumptions,
            warnings,
            sources: if rule_ids.is_empty() {
                registry.all_sources()
            } else {
                registry.sources(rule_ids)
            },
        },
    }
}

fn ensure_object(args: &Value) -> Result<(), FiscalError> {
    if args.is_object() {
        Ok(())
    } else {
        Err(invalid("arguments", "un objet JSON est requis"))
    }
}

fn invalid(name: &str, reason: &str) -> FiscalError {
    FiscalError::InvalidArgument {
        name: name.to_owned(),
        reason: reason.to_owned(),
    }
}

fn number(args: &Value, name: &str, default: Option<f64>) -> Result<f64, FiscalError> {
    let value = match args.get(name) {
        Some(Value::Number(v)) => v
            .as_f64()
            .ok_or_else(|| invalid(name, "nombre non représentable"))?,
        Some(Value::Null) | None => default.ok_or_else(|| invalid(name, "valeur requise"))?,
        _ => return Err(invalid(name, "un nombre est requis")),
    };
    if !value.is_finite() || !(0.0..=MAX_AMOUNT).contains(&value) {
        return Err(invalid(
            name,
            "la valeur doit être comprise entre 0 et 100 milliards",
        ));
    }
    Ok(value)
}

fn integer(args: &Value, name: &str, default: i64) -> Result<i64, FiscalError> {
    let value = args.get(name).and_then(Value::as_i64).unwrap_or(default);
    if !(0..=1_000_000).contains(&value) {
        return Err(invalid(name, "entier hors limites"));
    }
    Ok(value)
}

fn text<'a>(args: &'a Value, name: &str, default: &'a str) -> &'a str {
    args.get(name).and_then(Value::as_str).unwrap_or(default)
}

fn boolean(args: &Value, name: &str, default: bool) -> bool {
    args.get(name).and_then(Value::as_bool).unwrap_or(default)
}

fn round_euro(value: f64) -> f64 {
    value.max(0.0).round()
}
fn round_cents(value: f64) -> f64 {
    (value.max(0.0) * 100.0).round() / 100.0
}

fn base_parts(situation: &str) -> f64 {
    if matches!(situation, "marie" | "pacse") {
        2.0
    } else {
        1.0
    }
}

fn tax_parts(args: &Value) -> Result<f64, FiscalError> {
    if let Some(custom) = args.get("nb_parts_custom").and_then(Value::as_f64) {
        if (0.5..=20.0).contains(&custom) {
            return Ok(custom);
        }
        return Err(invalid(
            "nb_parts_custom",
            "doit être compris entre 0,5 et 20",
        ));
    }
    let situation = text(args, "situation_famille", "celibataire");
    let children = integer(args, "nb_enfants", 0)?;
    let alternating = integer(args, "nb_enfants_garde_alternee", 0)?;
    let disabled = integer(args, "enfants_handicap", 0)?;
    let mut parts = base_parts(situation);
    let mut rank = 0;
    for _ in 0..children {
        rank += 1;
        parts += if rank <= 2 { 0.5 } else { 1.0 };
    }
    for _ in 0..alternating {
        rank += 1;
        parts += if rank <= 2 { 0.25 } else { 0.5 };
    }
    parts += disabled as f64 * 0.5;
    if matches!(situation, "celibataire" | "divorce") && children + alternating > 0 {
        parts += 0.5;
    }
    Ok(parts)
}

fn bracket_ids(year: u16) -> [String; 4] {
    [0, 1, 2, 3].map(|index| format!("ir.{year}.bracket.{index}.ceiling"))
}

fn progressive_tax(income: f64, parts: f64, year: u16) -> (f64, f64) {
    let registry = Registry::global();
    let ids = bracket_ids(year);
    let ceilings = ids.map(|id| registry.number(&id));
    let rates = [
        0.0,
        registry.number("ir.scale.rate.1"),
        registry.number("ir.scale.rate.2"),
        registry.number("ir.scale.rate.3"),
        registry.number("ir.scale.rate.4"),
    ];
    let per_share = income / parts;
    let bounds = [0.0, ceilings[0], ceilings[1], ceilings[2], ceilings[3]];
    let mut tax = 0.0;
    let mut marginal = 0.0;
    for index in 0..5 {
        let upper = if index < 4 {
            bounds[index + 1]
        } else {
            f64::INFINITY
        };
        let base = (per_share.min(upper) - bounds[index]).max(0.0);
        tax += base * rates[index];
        if per_share > bounds[index] {
            marginal = rates[index];
        }
    }
    (tax * parts, marginal)
}

fn income_tax(args: &Value) -> Result<IncomeTaxResult, FiscalError> {
    let income = number(args, "revenu_net_imposable", None)?;
    let year = integer(args, "annee", 2026)? as u16;
    if !matches!(year, 2025 | 2026) {
        return Err(invalid(
            "annee",
            "seules 2025 et 2026 sont prises en charge",
        ));
    }
    let parts = tax_parts(args)?;
    let situation = text(args, "situation_famille", "celibataire");
    let base = base_parts(situation);
    let (full_tax, marginal) = progressive_tax(income, parts, year);
    let (base_tax, _) = progressive_tax(income, base, year);
    let cap = Registry::global().number(&format!("ir.{year}.family_quotient.half_share_cap"));
    let max_advantage = ((parts - base).max(0.0) / 0.5) * cap;
    let after_cap = full_tax.max(base_tax - max_advantage);
    let cap_adjustment = (after_cap - full_tax).max(0.0);

    let (discount, after_discount) = if year == 2026 {
        let couple = matches!(situation, "marie" | "pacse");
        let threshold = Registry::global().number(if couple {
            "ir.2026.decote.couple.threshold"
        } else {
            "ir.2026.decote.single.threshold"
        });
        let base_discount = Registry::global().number(if couple {
            "ir.2026.decote.couple.base"
        } else {
            "ir.2026.decote.single.base"
        });
        let rate = Registry::global().number("ir.2026.decote.rate");
        let discount = if after_cap < threshold {
            (base_discount - rate * after_cap).clamp(0.0, after_cap)
        } else {
            0.0
        };
        (discount, (after_cap - discount).max(0.0))
    } else {
        (0.0, after_cap)
    };

    Ok(IncomeTaxResult {
        income_tax_before_reliefs: round_euro(full_tax),
        family_quotient_cap_adjustment: round_euro(cap_adjustment),
        discount: round_euro(discount),
        income_tax: round_euro(after_discount),
        average_rate_percent: if income > 0.0 {
            round_cents(after_discount / income * 100.0)
        } else {
            0.0
        },
        marginal_rate_percent: marginal * 100.0,
        tax_shares: parts,
        taxable_income_per_share: round_euro(income / parts),
    })
}

fn calculate_income_tax_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let year = integer(args, "annee", 2026)? as u16;
    let result = income_tax(args)?;
    let summary = format!(
        "Impôt sur le revenu estimé : **{:.0} €**, pour {:.2} part(s), après plafonnement du quotient familial et décote applicables.",
        result.income_tax, result.tax_shares
    );
    let mut value = serde_json::to_value(result).expect("serializable");
    value["summary"] = Value::String(summary);
    let ids = bracket_ids(year);
    let mut refs: Vec<&str> = ids.iter().map(String::as_str).collect();
    refs.extend([
        "ir.scale.rate.1",
        "ir.scale.rate.2",
        "ir.scale.rate.3",
        "ir.scale.rate.4",
    ]);
    if year == 2026 {
        refs.extend([
            "ir.2026.family_quotient.half_share_cap",
            "ir.2026.decote.single.threshold",
            "ir.2026.decote.couple.threshold",
            "ir.2026.decote.rate",
        ]);
    }
    Ok(response(
        "Calcul de l’impôt sur le revenu",
        value,
        vec![
            "Simulation hors réductions, crédits d’impôt, CEHR et prélèvement à la source.".into(),
        ],
        if year == 2025 {
            vec!["La décote historique 2025 n’est pas appliquée : utilisez l’avis officiel pour une liquidation définitive.".into()]
        } else {
            vec![]
        },
        &refs,
    ))
}

fn simulate_brackets(args: &Value) -> Result<ToolResponse, FiscalError> {
    let gross = number(args, "revenu_annuel_brut", None)?;
    let kind = text(args, "type_revenu", "net_imposable");
    let (income, assumption) = match kind {
        "salaire_brut" => (
            gross * 0.77 * 0.90,
            "Conversion indicative salaire brut → net à 77 %, puis abattement de 10 %.",
        ),
        "salaire_net" => (
            gross * 0.90,
            "Application indicative de l’abattement forfaitaire de 10 %.",
        ),
        _ => (gross, "Montant traité comme revenu net imposable."),
    };
    let mock =
        json!({"revenu_net_imposable": income, "situation_famille": "celibataire", "annee": 2026});
    let tax = income_tax(&mock)?;
    Ok(response(
        "Simulation des tranches",
        json!({"summary": format!("Revenu net imposable retenu : **{income:.0} €** ; TMI : **{:.0} %**.", tax.marginal_rate_percent), "taxableIncome": round_euro(income), "marginalRatePercent": tax.marginal_rate_percent, "estimatedIncomeTax": tax.income_tax}),
        vec![assumption.into()],
        vec![
            "Les ratios brut/net sont indicatifs et dépendent du statut et des cotisations.".into(),
        ],
        &[
            "ir.2026.bracket.0.ceiling",
            "ir.2026.bracket.1.ceiling",
            "ir.2026.bracket.2.ceiling",
            "ir.2026.bracket.3.ceiling",
        ],
    ))
}

fn family_quotient_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let parts = tax_parts(args)?;
    Ok(response("Quotient familial", json!({"summary": format!("Nombre de parts estimé : **{parts:.2}**."), "taxShares": parts}), vec!["Les majorations spéciales (parent isolé, invalidité, ancien combattant) doivent être confirmées selon les cases de la déclaration.".into()], vec![], &["ir.2026.family_quotient.half_share_cap"]))
}

fn per_saving_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let payment = number(args, "montant_versement", None)?;
    let professional_income = number(args, "revenu_pro_net", Some(0.0))?;
    let minimum = Registry::global().number("per.2026.minimum");
    let maximum = Registry::global().number("per.2026.maximum");
    let ceiling = (professional_income * 0.10).clamp(minimum, maximum);
    let deductible = payment.min(ceiling);
    let base_args = json!({"revenu_net_imposable": number(args, "revenu_net_imposable", None)?, "situation_famille": text(args, "situation_famille", "celibataire"), "nb_enfants": integer(args, "nb_enfants", 0)?, "annee": 2026});
    let before = income_tax(&base_args)?.income_tax;
    let mut after_args = base_args;
    after_args["revenu_net_imposable"] =
        json!((number(args, "revenu_net_imposable", None)? - deductible).max(0.0));
    let after = income_tax(&after_args)?.income_tax;
    Ok(response(
        "Économie d’impôt PER",
        json!({"summary": format!("Versement déductible retenu : **{deductible:.0} €** ; économie d’IR estimée : **{:.0} €**.", before-after), "deductiblePayment": round_euro(deductible), "deductionCeiling": round_euro(ceiling), "estimatedTaxSaving": round_euro(before-after)}),
        vec!["Plafond personnel non consommé et reports antérieurs non fournis.".into()],
        vec!["Le plafond figurant sur l’avis d’impôt prévaut sur cette estimation.".into()],
        &["per.2026.minimum", "per.2026.maximum"],
    ))
}

fn ifi_amount(net: f64) -> f64 {
    if net <= Registry::global().number("ifi.2026.entry_threshold") {
        return 0.0;
    }
    let scale = Registry::global().number_array("ifi.2026.scale");
    let mut tax = scale
        .chunks_exact(3)
        .map(|bracket| (net.min(bracket[1]) - bracket[0]).max(0.0) * bracket[2])
        .sum::<f64>();
    let discount = Registry::global().number_array("ifi.2026.discount");
    if net >= discount[0] && net < discount[1] {
        tax -= (discount[2] - discount[3] * net).max(0.0);
    }
    tax.max(0.0)
}

fn ifi_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let gross = number(args, "patrimoine_immobilier_brut", None)?;
    let main_home = number(args, "valeur_residence_principale", Some(0.0))?;
    let debts = number(args, "dettes_deductibles", Some(0.0))?;
    let professional = number(args, "biens_professionnels", Some(0.0))?;
    let main_home_allowance = Registry::global().number("ifi.2026.main_home_allowance");
    let net = (gross - main_home * main_home_allowance - debts - professional).max(0.0);
    let tax = round_euro(ifi_amount(net));
    Ok(response("Impôt sur la fortune immobilière", json!({"summary": format!("Patrimoine net taxable : **{net:.0} €** ; IFI estimé : **{tax:.0} €**."), "netTaxableWealth": round_euro(net), "estimatedIfi": tax}), vec![format!("Abattement de {:.0} % appliqué à la résidence principale déclarée.", main_home_allowance * 100.0)], vec!["Les plafonnements de dettes et le plafonnement IFI/revenus nécessitent une analyse détaillée.".into()], &["ifi.2026.entry_threshold", "ifi.2026.scale", "ifi.2026.discount", "ifi.2026.main_home_allowance"]))
}

fn corporate_tax_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let profit =
        (number(args, "benefice", None)? - number(args, "deficit_reporte", Some(0.0))?).max(0.0);
    let ca = number(args, "ca", Some(0.0))?;
    let physical = number(args, "capital_personnes_physiques_pct", Some(0.0))?;
    let eligible = ca <= 10_000_000.0 && physical >= 75.0;
    let reduced_band = Registry::global().number("corporate_tax.2026.reduced_band");
    let reduced_rate = Registry::global().number("corporate_tax.2026.reduced_rate");
    let normal_rate = Registry::global().number("corporate_tax.2026.normal_rate");
    let reduced_base = if eligible {
        profit.min(reduced_band)
    } else {
        0.0
    };
    let tax = reduced_base * reduced_rate + (profit - reduced_base) * normal_rate;
    Ok(response(
        "Impôt sur les sociétés",
        json!({"summary": format!("IS estimé : **{tax:.0} €** sur un bénéfice taxable de **{profit:.0} €**."), "taxableProfit": round_euro(profit), "estimatedCorporateTax": round_euro(tax), "reducedRateEligible": eligible}),
        vec!["Capital supposé entièrement libéré ; condition à confirmer.".into()],
        vec![],
        &[
            "corporate_tax.2026.reduced_rate",
            "corporate_tax.2026.reduced_band",
            "corporate_tax.2026.normal_rate",
        ],
    ))
}

fn vat_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let ca = number(args, "chiffre_affaires_ht", Some(0.0))?;
    let activity = text(args, "type_activite", "services");
    let threshold_id = if activity.contains("vente")
        || activity.contains("commerce")
        || activity.contains("hebergement")
    {
        "vat.2026.sales.base_threshold"
    } else {
        "vat.2026.services.base_threshold"
    };
    let threshold = Registry::global().number(threshold_id);
    let collected = number(args, "tva_collectee", Some(0.0))?;
    let deductible = number(args, "tva_deductible", Some(0.0))?;
    let due = (collected - deductible).max(0.0);
    Ok(response("TVA", json!({"summary": format!("TVA nette déclarée : **{due:.0} €** ; seuil de franchise indicatif : **{threshold:.0} €**."), "netVatDue": round_euro(due), "baseFranchiseThreshold": threshold, "overBaseThreshold": ca > threshold}), vec![], vec!["Le dépassement des seuils majorés et les règles de l’année précédente peuvent modifier la date d’assujettissement.".into()], &[threshold_id]))
}

fn micro_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let ca = number(args, "chiffre_affaires_annuel", Some(0.0))?;
    let kind = text(args, "type_activite", "prestations_bnc");
    let sales = kind.contains("vente") || kind.contains("commerce") || kind.contains("hebergement");
    let threshold_id = if sales {
        "micro.2026.sales.threshold"
    } else {
        "micro.2026.services.threshold"
    };
    let rate_id = if sales {
        "micro.2026.social.sales_rate"
    } else if kind.contains("bic") {
        "micro.2026.social.services_bic_rate"
    } else if kind.contains("cipav") {
        "micro.2026.social.cipav_rate"
    } else {
        "micro.2026.social.bnc_rate"
    };
    let threshold = Registry::global().number(threshold_id);
    let rate = Registry::global().number(rate_id);
    let contributions = ca * rate;
    Ok(response("Micro-entrepreneur", json!({"summary": format!("Cotisations sociales estimées : **{contributions:.0} €** ({:.1} %) ; seuil micro : **{threshold:.0} €**.", rate*100.0), "turnover": ca, "socialContributions": round_euro(contributions), "socialRatePercent": rate*100.0, "microThreshold": threshold, "underThreshold": ca <= threshold}), vec!["Hors contribution à la formation professionnelle, taxe consulaire et impôt sur le revenu.".into()], vec!["La sortie du régime dépend en principe de deux années consécutives de dépassement.".into()], &[threshold_id, rate_id]))
}

fn pfu_comparison_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let amount = number(args, "montant", None)?;
    let pfu_rate = Registry::global().number("capital.2026.pfu_rate");
    let pfu = amount * pfu_rate;
    let base_income = number(args, "rni_autres_revenus", Some(0.0))?;
    let situation = text(args, "situation_famille", "celibataire");
    let children = integer(args, "nb_enfants", 0)?;
    let before_args = json!({"revenu_net_imposable": base_income, "situation_famille": situation, "nb_enfants": children, "annee": 2026});
    let after_args = json!({"revenu_net_imposable": base_income + amount, "situation_famille": situation, "nb_enfants": children, "annee": 2026});
    let progressive_ir =
        (income_tax(&after_args)?.income_tax - income_tax(&before_args)?.income_tax).max(0.0);
    let social_rate = Registry::global().number("capital.2026.social_rate");
    let scale = progressive_ir + amount * social_rate;
    Ok(response(
        "PFU ou barème progressif",
        json!({"summary": format!("PFU estimé : **{pfu:.0} €** ; barème + prélèvements sociaux : **{scale:.0} €**. Option indicative la moins coûteuse : **{}**.", if pfu <= scale {"PFU"} else {"barème"}), "pfuTax": round_euro(pfu), "progressiveTaxAndSocial": round_euro(scale), "preferredOption": if pfu <= scale {"pfu"} else {"progressive_scale"}}),
        vec![
            "Comparaison simplifiée sans CSG déductible, abattement sur dividendes ni CEHR.".into(),
        ],
        vec!["L’option pour le barème est globale pour les revenus mobiliers de l’année.".into()],
        &[
            "capital.2026.pfu_rate",
            "ir.2026.bracket.0.ceiling",
            "ir.2026.bracket.1.ceiling",
            "ir.2026.bracket.2.ceiling",
            "ir.2026.bracket.3.ceiling",
        ],
    ))
}

fn capital_gain_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let gain = number(args, "montant_plus_value", None)?;
    let tax = gain * Registry::global().number("capital.2026.pfu_rate");
    Ok(response(
        "Plus-value",
        json!({"summary": format!("Prélèvement forfaitaire estimé : **{tax:.0} €** sur **{gain:.0} €** de plus-value."), "capitalGain": gain, "estimatedFlatTax": round_euro(tax)}),
        vec!["Traitement par défaut comme plus-value mobilière soumise au PFU 2026.".into()],
        vec![
            "Immobilier, actifs professionnels, PEA et crypto suivent des régimes distincts."
                .into(),
        ],
        &["capital.2026.pfu_rate"],
    ))
}

fn crypto_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let proceeds = number(args, "prix_total_cession", None)?;
    let portfolio = number(args, "valeur_portefeuille_avant_cession", None)?;
    let acquisition = number(args, "prix_acquisition_moyen_portefeuille", None)?;
    let previous_losses = number(args, "moins_values_anterieures", Some(0.0))?;
    let gain = if portfolio > 0.0 {
        proceeds - acquisition * proceeds / portfolio
    } else {
        0.0
    };
    let taxable = (gain - previous_losses).max(0.0);
    let tax = taxable * Registry::global().number("capital.2026.pfu_rate");
    Ok(response("Fiscalité des crypto-actifs", json!({"summary": format!("Plus-value de cession calculée : **{gain:.0} €** ; impôt indicatif : **{tax:.0} €**."), "capitalGain": round_euro(gain), "taxableCapitalGain": round_euro(taxable), "estimatedTax": round_euro(tax)}), vec!["Cession supposée réalisée contre monnaie ayant cours légal, bien ou service.".into()], vec!["Les échanges crypto→crypto sans soulte ne sont pas, à eux seuls, imposables ; les moins-values des particuliers ne sont pas reportables dix ans.".into()], &["capital.2026.pfu_rate"]))
}

fn property_gain_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let sale = number(args, "prix_vente", None)?;
    let purchase = number(args, "prix_achat", Some(0.0))?;
    let costs = number(args, "frais_achat", Some(0.0))?;
    let works = number(args, "travaux_justifies", Some(0.0))?;
    let years = number(args, "duree_detention_ans", Some(0.0))?;
    let raw = (sale - purchase - costs - works).max(0.0);
    let schedule = Registry::global().number_array("property_gain.2026.allowance_schedule");
    let exemption_years = Registry::global().number_array("property_gain.2026.exemption_years");
    let ir_reduction = if years <= schedule[0] {
        0.0
    } else if years <= schedule[1] {
        (years - schedule[0]) * schedule[2]
    } else if years < exemption_years[0] {
        (schedule[1] - schedule[0]) * schedule[2] + (years - schedule[1]) * schedule[3]
    } else {
        1.0
    };
    let social_reduction = if years <= schedule[0] {
        0.0
    } else if years <= schedule[1] {
        (years - schedule[0]) * schedule[4]
    } else if years <= exemption_years[0] {
        (schedule[1] - schedule[0]) * schedule[4] + (years - schedule[1]) * schedule[5]
    } else if years < exemption_years[1] {
        (schedule[1] - schedule[0]) * schedule[4]
            + schedule[5]
            + (years - exemption_years[0]) * schedule[6]
    } else {
        1.0
    };
    let tax = raw
        * (1.0 - ir_reduction.min(1.0))
        * Registry::global().number("property_gain.2026.income_tax_rate")
        + raw
            * (1.0 - social_reduction.min(1.0))
            * Registry::global().number("property_gain.2026.social_rate");
    Ok(response("Plus-value immobilière", json!({"summary": format!("Plus-value brute : **{raw:.0} €** ; impôt et prélèvements estimés : **{tax:.0} €**."), "grossGain": round_euro(raw), "estimatedTaxAndSocial": round_euro(tax), "incomeTaxAllowancePercent": round_cents(ir_reduction.min(1.0)*100.0), "socialAllowancePercent": round_cents(social_reduction.min(1.0)*100.0)}), vec!["Hors surtaxe sur plus-values élevées et exonérations particulières.".into()], vec!["Confirmer le régime exact auprès du notaire, notamment pour une ancienne activité LMNP.".into()], &["property_gain.2026.income_tax_rate", "property_gain.2026.social_rate", "property_gain.2026.exemption_years", "property_gain.2026.allowance_schedule"]))
}

fn micro_foncier_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let rent = number(args, "loyers_bruts_annuels", None)?;
    let charges = [
        "interets_emprunt",
        "charges_copropriete",
        "taxe_fonciere",
        "travaux_entretien_annuels",
        "frais_gestion_annuels",
        "assurance_pno",
    ]
    .iter()
    .try_fold(0.0, |sum, name| {
        Ok::<_, FiscalError>(sum + number(args, name, Some(0.0))?)
    })?;
    let allowance = Registry::global().number("rental.micro_foncier.allowance");
    let threshold = Registry::global().number("rental.micro_foncier.threshold");
    let micro_base = rent * (1.0 - allowance);
    let real_base = (rent - charges).max(0.0);
    Ok(response(
        "Micro-foncier ou régime réel",
        json!({"summary": format!("Base micro-foncier : **{micro_base:.0} €** ; base au réel : **{real_base:.0} €**."), "microTaxableIncome": round_euro(micro_base), "actualTaxableIncome": round_euro(real_base), "preferredRegime": if micro_base <= real_base {"micro_foncier"} else {"real"}}),
        vec![format!(
            "Abattement micro-foncier de {:.0} % appliqué sous le seuil de {:.0} € si le foyer y est éligible.",
            allowance * 100.0,
            threshold
        )],
        vec!["L’option pour le réel engage en principe pour trois ans.".into()],
        &[
            "rental.micro_foncier.threshold",
            "rental.micro_foncier.allowance",
        ],
    ))
}

fn lmnp_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let rents = number(args, "loyers_annuels_bruts", None)?;
    let charges = number(args, "charges_annuelles", Some(0.0))?
        + number(args, "interets_emprunt_annuels", Some(0.0))?
        + number(args, "taxe_fonciere", Some(0.0))?;
    let building = (number(args, "valeur_bien_hors_terrain", Some(0.0))?
        - number(args, "valeur_terrain", Some(0.0))?)
    .max(0.0);
    let furniture = number(args, "valeur_mobilier", Some(0.0))?;
    let estimated_depreciation = building / 30.0 + furniture / 7.0;
    let taxable = (rents - charges - estimated_depreciation).max(0.0);
    Ok(response("Location meublée non professionnelle", json!({"summary": format!("Résultat LMNP réel indicatif après amortissements : **{taxable:.0} €**."), "rentalIncome": rents, "deductibleCharges": round_euro(charges), "estimatedDepreciation": round_euro(estimated_depreciation), "estimatedTaxableIncome": round_euro(taxable)}), vec!["Amortissements linéaires indicatifs sur 30 ans pour l’immeuble et 7 ans pour le mobilier.".into()], vec!["Depuis la réforme applicable, certains amortissements LMNP peuvent être réintégrés dans le calcul de la plus-value de cession ; demander un tableau comptable.".into()], &[]))
}

fn scpi_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let invested = number(args, "montant_investi", None)?;
    let yield_percent = number(args, "rendement_brut_pct", None)?;
    let gross_income = invested * yield_percent / 100.0;
    Ok(response(
        "SCPI",
        json!({"summary": format!("Revenu foncier brut indicatif : **{gross_income:.0} € par an**."), "investedAmount": invested, "grossAnnualIncome": round_euro(gross_income), "grossYieldPercent": yield_percent}),
        vec!["Rendement constant et absence de vacance supposés.".into()],
        vec![
            "Simulation de marché indicative ; ni le rendement ni le capital ne sont garantis."
                .into(),
        ],
        &[],
    ))
}

fn pea_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let paid = number(args, "versements_cumules", Some(0.0))?;
    let value = number(args, "valeur_actuelle", Some(0.0))?;
    let withdrawal = number(args, "montant_retrait", Some(0.0))?;
    let years = number(args, "anciennete_ans", Some(0.0))?;
    let gain_ratio = if value > 0.0 {
        ((value - paid).max(0.0)) / value
    } else {
        0.0
    };
    let gain = withdrawal * gain_ratio;
    let limits = Registry::global().number_array("pea.2026.payment_limits");
    let exemption_years = Registry::global().number("pea.2026.income_tax_exemption_years");
    let tax = if years >= exemption_years {
        gain * Registry::global().number("capital.2026.social_rate")
    } else {
        gain * Registry::global().number("capital.2026.pfu_rate")
    };
    let limit = if text(args, "type_pea", "classique").contains("pme") {
        limits[1]
    } else {
        limits[0]
    };
    Ok(response(
        "PEA",
        json!({"summary": format!("Gain inclus dans le retrait : **{gain:.0} €** ; prélèvements estimés : **{tax:.0} €**."), "gainInWithdrawal": round_euro(gain), "estimatedTaxAndSocial": round_euro(tax), "paymentLimit": limit, "combinedPaymentLimit": limits[2]}),
        vec![],
        vec!["Le plafond cumulé PEA + PEA-PME doit aussi être respecté.".into()],
        &[
            "capital.2026.pfu_rate",
            "capital.2026.social_rate",
            "pea.2026.payment_limits",
            "pea.2026.income_tax_exemption_years",
        ],
    ))
}

fn life_insurance_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let capital = number(args, "capital_total", None)?;
    let paid = number(args, "versements_cumules", None)?;
    let withdrawal = number(args, "montant_rachat", Some(0.0))?;
    let years = number(args, "anciennete_ans", Some(0.0))?;
    let gain = if capital > 0.0 {
        withdrawal * ((capital - paid).max(0.0) / capital)
    } else {
        0.0
    };
    let allowances = Registry::global().number_array("life_insurance.2026.allowances");
    let allowance = if matches!(
        text(args, "situation_famille", "celibataire"),
        "marie" | "pacse"
    ) {
        allowances[1]
    } else {
        allowances[0]
    };
    let taxable = if years >= allowances[2] {
        (gain - allowance).max(0.0)
    } else {
        gain
    };
    let tax = taxable * Registry::global().number("capital.2026.income_tax_rate")
        + gain * Registry::global().number("capital.2026.social_rate");
    Ok(response(
        "Assurance-vie",
        json!({"summary": format!("Part de gains du rachat : **{gain:.0} €** ; fiscalité indicative : **{tax:.0} €**."), "gainInWithdrawal": round_euro(gain), "taxableGainAfterAllowance": round_euro(taxable), "estimatedTaxAndSocial": round_euro(tax)}),
        vec!["Primes supposées versées après le 27 septembre 2017.".into()],
        vec![
            "L’antériorité des primes et le total des encours peuvent changer le taux d’IR.".into(),
        ],
        &[
            "life_insurance.2026.allowances",
            "capital.2026.income_tax_rate",
            "capital.2026.social_rate",
        ],
    ))
}

fn per_exit_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let capital = number(args, "capital_total", None)?;
    let paid = number(args, "versements_cumules", None)?;
    let deducted = number(args, "versements_deduits", Some(0.0))?.min(paid);
    let gains = (capital - paid).max(0.0);
    let social = gains * Registry::global().number("capital.2026.social_rate");
    Ok(response(
        "Sortie du PER",
        json!({"summary": format!("Capital correspondant aux versements déduits : **{deducted:.0} €** ; gains : **{gains:.0} €**."), "deductedPaymentsTaxableAtScale": round_euro(deducted), "capitalGains": round_euro(gains), "socialContributionsOnGains": round_euro(social)}),
        vec!["Sortie en capital au départ à la retraite.".into()],
        vec!["Le taux d’IR dépendra des autres revenus de l’année de sortie.".into()],
        &[
            "per.2026.minimum",
            "per.2026.maximum",
            "capital.2026.social_rate",
        ],
    ))
}

fn withholding_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let income = number(args, "revenu_net_imposable", None)?;
    let mock = json!({"revenu_net_imposable": income, "situation_famille": text(args, "situation_famille", "celibataire"), "nb_enfants": integer(args, "nb_enfants", 0)?, "annee": 2026});
    let annual = income_tax(&mock)?.income_tax;
    let rate = if income > 0.0 { annual / income } else { 0.0 };
    let salary = number(args, "salaire_mensuel_net", Some(income / 12.0))?;
    Ok(response(
        "Prélèvement à la source",
        json!({"summary": format!("Taux personnalisé indicatif : **{:.2} %** ; retenue mensuelle : **{:.0} €**.", rate*100.0, salary*rate), "estimatedRatePercent": round_cents(rate*100.0), "estimatedMonthlyWithholding": round_euro(salary*rate)}),
        vec!["Approximation fondée sur l’IR annuel avant réductions et crédits.".into()],
        vec!["Le taux officiel transmis par la DGFiP prévaut.".into()],
        &[
            "ir.2026.bracket.0.ceiling",
            "ir.2026.bracket.1.ceiling",
            "ir.2026.bracket.2.ceiling",
            "ir.2026.bracket.3.ceiling",
        ],
    ))
}

fn cfe_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let first_year = boolean(args, "premiere_annee_activite", false);
    let rental_value = number(args, "valeur_locative_brute", Some(0.0))?;
    Ok(response("Cotisation foncière des entreprises", json!({"summary": if first_year {"Exonération de CFE de l’année de création à confirmer.".to_owned()} else {"Le taux communal manque : aucun montant national artificiel n’est calculé.".to_owned()}, "estimatedCfe": Value::Null, "rentalValue": rental_value, "firstYearExemption": first_year}), vec![], vec!["Les bases minimum et les taux 2026 dépendent de la commune ; consulter l’avis ou la délibération locale officielle.".into()], &[]))
}

fn exceptional_income_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let ordinary = number(args, "rni_ordinaire", Some(0.0))?;
    let exceptional = number(args, "revenu_exceptionnel", None)?;
    let divisor = number(args, "nombre_annees_echelement", Some(4.0))?.clamp(1.0, 4.0);
    let situation = text(args, "situation_famille", "celibataire");
    let children = integer(args, "nb_enfants", 0)?;
    let base = json!({"revenu_net_imposable": ordinary, "situation_famille": situation, "nb_enfants": children, "annee": 2026});
    let quotient = json!({"revenu_net_imposable": ordinary + exceptional/divisor, "situation_famille": situation, "nb_enfants": children, "annee": 2026});
    let extra =
        (income_tax(&quotient)?.income_tax - income_tax(&base)?.income_tax).max(0.0) * divisor;
    Ok(response(
        "Revenus exceptionnels — système du quotient",
        json!({"summary": format!("Surcroît d’IR estimé par quotient : **{extra:.0} €**."), "estimatedAdditionalTax": round_euro(extra), "quotientDivisor": divisor}),
        vec!["Le revenu est supposé éligible au système du quotient.".into()],
        vec!["L’éligibilité dépend de la nature et du caractère exceptionnel du revenu.".into()],
        &[
            "ir.2026.bracket.0.ceiling",
            "ir.2026.bracket.1.ceiling",
            "ir.2026.bracket.2.ceiling",
            "ir.2026.bracket.3.ceiling",
        ],
    ))
}

fn replacement_income_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let amount = number(
        args,
        "montant",
        Some(number(args, "remuneration_annuelle_brute", Some(0.0))?),
    )?;
    Ok(response("Revenus de remplacement", json!({"summary": format!("Montant analysé : **{amount:.0} €**. Le régime dépend de la nature exacte du revenu."), "amount": amount, "incomeType": text(args, "type_revenu", "non_precise")}), vec![], vec!["Les indemnités de rupture, pensions, rentes et allocations ont des règles distinctes ; résultat documentaire, non liquidation définitive.".into()], &[]))
}

fn donation_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let amount = number(args, "montant_donation", None)?;
    let relationship = text(args, "lien_parente", "autre");
    let allowances = Registry::global().number_array("donation.2026.allowances");
    let allowance = match relationship {
        "enfant" | "parent" => allowances[0],
        "petit_enfant" => allowances[1],
        "frere_soeur" => allowances[2],
        "neveu_niece" => allowances[3],
        "epoux" | "pacse" => allowances[4],
        _ => allowances[5],
    };
    let previous = number(args, "donations_anterieures", Some(0.0))?;
    let taxable = (amount - (allowance - previous).max(0.0)).max(0.0);
    Ok(response("Droits de donation", json!({"summary": format!("Base taxable après abattement indicatif : **{taxable:.0} €**."), "allowance": allowance, "taxableGift": round_euro(taxable)}), vec![format!("Abattement personnel supposé renouvelé sur une période de {:.0} ans.", Registry::global().number("donation.allowance_renewal_years"))], vec!["Le barème des droits n’est pas liquidé ici ; vérifier les donations antérieures et conditions du don familial de somme d’argent.".into()], &["donation.2026.allowances", "donation.allowance_renewal_years"]))
}

fn succession_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let estate = number(args, "actif_net_succession", None)?;
    let life = number(args, "assurance_vie_hors_succession", Some(0.0))?;
    Ok(response("Succession", json!({"summary": format!("Actif successoral net analysé : **{estate:.0} €** ; assurance-vie hors succession déclarée : **{life:.0} €**."), "netEstate": estate, "lifeInsuranceOutsideEstate": life}), vec![], vec!["Les droits exigent la ventilation par héritier, le lien de parenté, les donations antérieures et le testament.".into()], &[]))
}

fn bare_ownership_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let value = number(args, "valeur_pleine_propriete", None)?;
    let age = integer(args, "age_usufruitier", 0)?;
    let usufruct_percent = match age {
        0..=20 => 90.0,
        21..=30 => 80.0,
        31..=40 => 70.0,
        41..=50 => 60.0,
        51..=60 => 50.0,
        61..=70 => 40.0,
        71..=80 => 30.0,
        81..=90 => 20.0,
        _ => 10.0,
    };
    let bare = value * (100.0 - usufruct_percent) / 100.0;
    Ok(response(
        "Démembrement",
        json!({"summary": format!("Valeur fiscale indicative de la nue-propriété : **{bare:.0} €**."), "usufructPercent": usufruct_percent, "bareOwnershipValue": round_euro(bare)}),
        vec!["Barème viager fondé sur l’âge révolu de l’usufruitier.".into()],
        vec![
            "L’usufruit temporaire obéit à une autre règle et doit être traité séparément.".into(),
        ],
        &[],
    ))
}

fn sci_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let rents = number(args, "loyers_annuels", None)?;
    let charges =
        number(args, "charges_annuelles", Some(0.0))? + number(args, "interet_emprunt", Some(0.0))?;
    let share = number(args, "parts_contribuable", Some(100.0))?.min(100.0) / 100.0;
    let result = (rents - charges) * share;
    Ok(response(
        "SCI",
        json!({"summary": format!("Quote-part de résultat foncier avant impôt : **{result:.0} €**."), "partnerShareResult": result, "sharePercent": share*100.0}),
        vec!["SCI supposée translucide à l’IR.".into()],
        vec!["Une SCI à l’IS implique amortissements et fiscalité de cession différentes.".into()],
        &[],
    ))
}

fn business_sale_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let price = number(args, "prix_cession", None)?;
    let acquisition = number(args, "prix_acquisition", Some(0.0))?;
    let gain = (price - acquisition).max(0.0);
    let tax = gain * Registry::global().number("capital.2026.pfu_rate");
    Ok(response("Cession d’entreprise", json!({"summary": format!("Plus-value brute : **{gain:.0} €** ; PFU de référence : **{tax:.0} €**."), "grossCapitalGain": round_euro(gain), "referenceFlatTax": round_euro(tax)}), vec!["Cession de titres par une personne physique supposée.".into()], vec!["Départ à la retraite, durée de détention, apport-cession et titres de PME peuvent changer le résultat.".into()], &["capital.2026.pfu_rate"]))
}

fn holding_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let profit = number(args, "benefice_filiale", None)?;
    let ownership = number(args, "taux_detention_holding", Some(100.0))?.min(100.0) / 100.0;
    let dividend = profit * ownership;
    let addback_rate = Registry::global().number("holding.2026.parent_subsidiary_addback");
    let taxable_parent_child = dividend * addback_rate;
    Ok(response("Holding", json!({"summary": format!("Dividende remonté : **{dividend:.0} €** ; quote-part taxable indicative : **{taxable_parent_child:.0} €**."), "upstreamDividend": round_euro(dividend), "estimatedTaxableAddBack": round_euro(taxable_parent_child)}), vec!["Régime mère-fille supposé applicable et conditions de détention remplies.".into()], vec!["Simulation indicative ; l’intégration fiscale et les distributions personnelles nécessitent une analyse séparée.".into()], &["holding.2026.parent_subsidiary_addback"]))
}

fn foreign_income_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let france = number(args, "revenu_france", Some(0.0))?;
    let foreign = number(args, "revenu_etranger_eur", None)?;
    let paid = number(args, "impot_paye_etranger", Some(0.0))?;
    let mock = json!({"revenu_net_imposable": france+foreign, "situation_famille": text(args, "situation_famille", "celibataire"), "nb_enfants": integer(args, "nb_enfants", 0)?, "annee": 2026});
    let french_tax = income_tax(&mock)?.income_tax;
    Ok(response("Revenus étrangers", json!({"summary": format!("IR français avant mécanisme conventionnel : **{french_tax:.0} €** ; impôt étranger déclaré : **{paid:.0} €**."), "worldwideIncome": round_euro(france+foreign), "frenchTaxBeforeTreatyRelief": french_tax, "foreignTaxPaid": paid}), vec!["Résident fiscal de France supposé.".into()], vec!["La convention fiscale bilatérale détermine exemption ou crédit d’impôt ; aucune conclusion sans pays et article conventionnel vérifiés.".into()], &["ir.2026.bracket.0.ceiling", "ir.2026.bracket.1.ceiling", "ir.2026.bracket.2.ceiling", "ir.2026.bracket.3.ceiling"]))
}

fn exit_tax_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let gains = number(args, "plus_values_latentes_total", None)?;
    let years = integer(args, "annees_residence_france_10_dernieres", 0)?;
    let thresholds = Registry::global().number_array("exit_tax.2026.thresholds");
    let threshold_trigger = gains >= thresholds[0];
    let residence_trigger = years as f64 >= thresholds[2];
    let potentially_subject = residence_trigger && threshold_trigger;
    Ok(response(
        "Exit tax",
        json!({"summary": format!("Assujettissement potentiel selon les données partielles : **{}**.", if potentially_subject {"oui"} else {"non ou indéterminé"}), "potentiallySubject": if potentially_subject {Value::Bool(true)} else {Value::Null}, "latentGains": gains}),
        vec![
            "Valeur globale des titres et pourcentage de participation non fournis séparément."
                .into(),
        ],
        vec![format!(
            "Le déclencheur alternatif tenant à une participation d’au moins {:.0} % doit être contrôlé même sous {:.0} €.",
            thresholds[1] * 100.0,
            thresholds[0]
        )],
        &["exit_tax.2026.thresholds"],
    ))
}

fn employee_savings_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let amount = number(args, "montant", Some(0.0))?;
    let employer = number(args, "abondement_employeur", Some(0.0))?;
    Ok(response(
        "Épargne salariale",
        json!({"summary": format!("Épargne investie : **{amount:.0} €** ; abondement employeur : **{employer:.0} €**."), "employeePayment": amount, "employerMatch": employer, "totalInvested": round_euro(amount+employer)}),
        vec!["Sommes bloquées selon le dispositif déclaré.".into()],
        vec![
            "Les plafonds d’abondement et forfaits sociaux dépendent du plan et de l’effectif."
                .into(),
        ],
        &[],
    ))
}

fn retirement_tool(tool: &str, args: &Value) -> Result<ToolResponse, FiscalError> {
    let amount = args
        .get("salaire_annuel_brut")
        .or_else(|| args.get("pension_annuelle_defunt"))
        .or_else(|| args.get("nb_trimestres_racheter"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    Ok(response("Retraite", json!({"summary": "Analyse retraite fournie comme orientation : les paramètres générationnels doivent être vérifiés sur le relevé de carrière.", "tool": tool, "referenceAmount": amount}), vec![], vec!["L’âge légal et le nombre de trimestres requis varient selon la date de naissance ; aucun couple 64 ans/172 trimestres n’est appliqué universellement.".into()], &["pass.2026.annual"]))
}

fn diagnostic_tool(tool: &str, args: &Value) -> Result<ToolResponse, FiscalError> {
    let income = args
        .get("revenu_net_imposable")
        .or_else(|| args.get("salaire_net_annuel"))
        .or_else(|| args.get("salaire_brut_annuel_cdi"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let mut opportunities = Vec::new();
    if income > 0.0 {
        opportunities
            .push("Comparer le prélèvement à la source au revenu prévisionnel.".to_owned());
    }
    if number(args, "versements_per_annuels", Some(0.0))? > 0.0 || boolean(args, "a_per", false) {
        opportunities.push("Contrôler le plafond PER personnel sur l’avis 2026.".to_owned());
    }
    opportunities.push(
        "Vérifier les crédits et réductions sur justificatifs officiels avant déclaration."
            .to_owned(),
    );
    Ok(response(
        "Diagnostic fiscal",
        json!({"summary": format!("Diagnostic **{tool}** : {} piste(s) de contrôle identifiée(s).", opportunities.len()), "opportunities": opportunities}),
        vec!["Diagnostic non exhaustif fondé uniquement sur les champs fournis.".into()],
        vec!["Aucune stratégie n’est recommandée sans coût, risque, liquidité et horizon.".into()],
        &[],
    ))
}

fn guide_tool(tool: &str, args: &Value) -> Result<ToolResponse, FiscalError> {
    let title = tool.replace('_', " ");
    let registry = Registry::global();
    let (result, assumptions, warnings, rules): (Value, Vec<String>, Vec<String>, Vec<&str>) =
        match tool {
            "lister_credits_impot" => {
                let home = registry.number_array("tax_credit.2026.home_employment");
                let childcare = registry.number_array("tax_credit.2026.childcare");
                (json!({"summary": "Principaux crédits 2026 : emploi à domicile et garde d’enfant hors domicile.", "credits": [{"id": "home_employment", "ratePercent": home[0]*100.0, "standardExpenseLimit": home[1], "firstYearExpenseLimit": home[2]}, {"id": "childcare_under_six", "ratePercent": childcare[0]*100.0, "expenseLimitPerChild": childcare[1], "maximumCreditPerChild": childcare[2]}]}), vec![], vec!["Liste principale, non exhaustive ; l’éligibilité dépend des justificatifs et de la composition du foyer.".into()], vec!["tax_credit.2026.home_employment", "tax_credit.2026.childcare"])
            }
            "lister_reductions_impot" => {
                let donations = registry.number_array("tax_reduction.2026.donations");
                let caps = registry.number_array("tax_advantages.2026.global_cap");
                (json!({"summary": "Réductions 2026 documentées : dons et plafond global des avantages.", "donations": {"ordinaryRatePercent": donations[0]*100.0, "incomeLimitPercent": donations[1]*100.0, "aidOrganizationsRatePercent": donations[2]*100.0, "aidOrganizationsLimit": donations[3]}, "globalCap": caps[0], "specialCap": caps[1]}), vec![], vec!["Les dons sont hors plafond global ; investissements et dispositifs territoriaux exigent leur propre période de souscription.".into()], vec!["tax_reduction.2026.donations", "tax_advantages.2026.global_cap"])
            }
            "lister_deductions_revenu" => (json!({"summary": "Déductions à contrôler : frais professionnels, pensions alimentaires admises et versements PER dans le plafond personnel.", "deductions": ["professional_expenses", "eligible_alimony", "per_payments"]}), vec![], vec!["Le montant PER exact figurant sur l’avis d’impôt prévaut.".into()], vec!["per.2026.minimum", "per.2026.maximum"]),
            "lister_epargne_defiscalisante" => (json!({"summary": "Épargne réglementée au 1er août 2026 : Livret A/LDDS à 1,7 %, LEP à 2,5 % ; PEA et PER selon leurs règles propres.", "products": [{"id":"livret_a_ldds", "ratePercent": registry.number("savings.2026.livret_a_rate")*100.0}, {"id":"lep", "ratePercent": registry.number("savings.2026.lep_rate")*100.0}, {"id":"pea"}, {"id":"per"}]}), vec![], vec!["Le rendement futur n’est pas garanti et les taux réglementés peuvent changer à la prochaine période.".into()], vec!["savings.2026.livret_a_rate", "savings.2026.lep_rate", "pea.2026.payment_limits", "per.2026.minimum", "per.2026.maximum"]),
            "guide_frais_reels" => {
                let distance = number(args, "distance_domicile_travail_km", Some(0.0))?;
                let days = number(args, "nb_jours_travail", Some(0.0))?;
                (json!({"summary": format!("Distance annuelle aller-retour déclarée : **{:.0} km**. Appliquer ensuite le barème kilométrique officiel correspondant au véhicule.", distance*2.0*days), "annualRoundTripKilometers": round_euro(distance*2.0*days), "requiresOfficialMileageScale": true}), vec!["Distance traitée comme un aller simple.".into()], vec!["Aucun tarif kilométrique n’est inventé : puissance fiscale, énergie et barème officiel de l’année sont requis.".into()], vec![])
            }
            "calendrier_fiscal" => {
                let events = registry.string_array("calendar.2026.income_tax");
                (json!({"summary": format!("Calendrier fiscal 2026 : **{} échéances IR** documentées.", events.len()), "events": events}), vec![], vec!["Les dates locales ou reportées par la DGFiP doivent être vérifiées sur l’espace particulier.".into()], vec!["calendar.2026.income_tax"])
            }
            "info_fiscalite_immobilier" => (json!({"summary": "Location nue : micro-foncier sous conditions ou réel. Location meublée : BIC avec règles micro/réel et amortissements.", "microFoncierThreshold": registry.number("rental.micro_foncier.threshold"), "microFoncierAllowancePercent": registry.number("rental.micro_foncier.allowance")*100.0}), vec![], vec!["Meublé touristique classé et non classé ont des seuils 2026 distincts.".into()], vec!["rental.micro_foncier.threshold", "rental.micro_foncier.allowance", "micro.2026.services.threshold"]),
            "analyser_declaration_revenus" => {
                let income_cases = ["case_1AJ", "case_1BJ", "case_2DC", "case_2TR", "case_3VG", "case_4BA", "case_4BE"];
                let gross = income_cases.iter().try_fold(0.0, |sum, field| Ok::<_, FiscalError>(sum + number(args, field, Some(0.0))?))?;
                let per = number(args, "case_6NS", Some(0.0))? + number(args, "case_6NT", Some(0.0))?;
                (json!({"summary": format!("Total mécanique des principales cases de revenus fournies : **{gross:.0} €** ; versements PER déclarés : **{per:.0} €**."), "declaredIncomeCasesTotal": round_euro(gross), "declaredPerPayments": round_euro(per)}), vec!["Somme de contrôle, pas reconstitution du revenu net imposable : chaque case conserve son régime.".into()], vec!["Ne pas additionner ce résultat à un RFR déjà calculé.".into()], vec!["per.2026.minimum", "per.2026.maximum"])
            }
            "guide_maprimerenov" => (json!({"summary": "MaPrimeRénov’ 2026 doit être simulée avec la commune, la taille du ménage, le RFR 2025, le parcours et les travaux exacts.", "status": "official_anah_simulator_required"}), vec![], vec!["Le schéma historique ne contient ni région ni nombre de personnes : aucun montant fiable n’est calculé.".into()], vec![]),
            "checker_eligibilite_aides" => (json!({"summary": "Pré-diagnostic d’aides : utiliser les simulateurs officiels CAF, France Travail, Anah et mesdroitssociaux.gouv.fr.", "status": "official_simulators_required"}), vec![], vec!["Les aides sont dynamiques et ne peuvent pas être certifiées à partir du seul RFR et du nombre de parts.".into()], vec![]),
            "comparer_scenarios" => {
                let count = args.get("scenarios").and_then(Value::as_array).map_or(0, Vec::len);
                (json!({"summary": format!("**{count} scénario(s)** reçu(s). Comparaison structurée disponible lorsque chaque scénario précise revenu, situation et hypothèses homogènes."), "scenarioCount": count}), vec![], vec!["Aucun scénario hétérogène n’est classé automatiquement sans même année et même périmètre.".into()], vec![])
            }
            "guide_fiscalite_internationale" | "guide_frontaliers" => (json!({"summary": "Identifier d’abord la résidence fiscale, la source du revenu, la période et l’article de la convention bilatérale applicable.", "country": text(args, if tool == "guide_frontaliers" {"pays_emploi"} else {"pays"}, "non_precise"), "status": "treaty_article_required"}), vec![], vec!["Aucun taux étranger ou accord de télétravail non daté n’est appliqué automatiquement.".into()], vec![]),
            "simuler_pacte_dutreil" => {
                let value = number(args, "valeur_entreprise", Some(0.0))?;
                let exempt_share = registry.number("dutreil.2026.exempt_share");
                (json!({"summary": format!("Assiette après exonération Dutreil de {:.0} % : **{:.0} €**, sous réserve de toutes les conditions.", exempt_share*100.0, value*(1.0-exempt_share)), "businessValue": value, "taxableBaseBeforePersonalAllowances": round_euro(value*(1.0-exempt_share))}), vec!["Entreprise et titres supposés éligibles.".into()], vec!["Engagements de conservation, activité opérationnelle et fonction de direction doivent être vérifiés.".into()], vec!["dutreil.2026.exempt_share"])
            }
            "guide_fiscalite_outremer" => (json!({"summary": "Les réductions et exonérations outre-mer dépendent du territoire, de la date, du secteur et de l’agrément.", "territory": text(args, "territoire", "non_precise")}), vec![], vec!["La TVA n’est provisoirement applicable ni en Guyane ni à Mayotte ; aucune affirmation contraire n’est utilisée.".into()], vec![]),
            "guide_taxe_fonciere" => (json!({"summary": "La taxe foncière exige la valeur locative cadastrale et les taux votés localement ; aucun taux national fictif n’est appliqué.", "providedRentalValue": number(args, "valeur_locative_brute", Some(0.0))?}), vec![], vec!["Consulter l’avis 2026 et les délibérations de la commune/EPCI.".into()], vec![]),
            "guide_loc_avantages" => (json!({"summary": "Loc’Avantages exige le plafond de loyer Anah de la commune, le niveau de convention et l’éventuelle intermédiation locative.", "status": "anah_local_ceiling_required"}), vec![], vec!["Aucun loyer de marché par zone n’est inventé.".into()], vec![]),
            "guide_revision_declaration" => (json!({"summary": "Correction en ligne 2026 ouverte depuis le 29 juillet jusqu’en décembre ; après fermeture, utiliser la réclamation selon le délai applicable.", "submitted": boolean(args, "declaration_deja_soumise", false)}), vec![], vec!["La procédure dépend de l’impôt et de la date de mise en recouvrement.".into()], vec!["calendar.2026.income_tax"]),
            "guide_evenements_vie" | "guide_fiscalite_agricole" => (json!({"summary": format!("Guide **{title}** : les règles applicables doivent être sélectionnées selon l’événement, le régime et leur date d’effet."), "status": "specialized_official_rules_required"}), vec![], vec!["Aucun seuil spécialisé non sourcé n’est utilisé.".into()], vec![]),
            _ => (json!({"summary": format!("Guide **{title}** actualisé au 1er août 2026. Les règles locales, conventionnelles ou soumises à agrément restent à confirmer."), "status": "official_sources_required_for_individual_decision", "tool": tool}), vec!["Réponse documentaire : aucun montant fiscal personnel n’est liquidé lorsqu’une donnée officielle déterminante manque.".into()], vec!["Simulation indicative pour les hypothèses de marché, territoriales, énergétiques ou de rendement.".into()], vec![]),
        };
    Ok(response(&title, result, assumptions, warnings, &rules))
}

fn freshness_tool(args: &Value) -> Result<ToolResponse, FiscalError> {
    let target = integer(args, "annee_cible", 2026)? as u16;
    let registry = Registry::global();
    let audited =
        NaiveDate::parse_from_str(registry.audited_at(), "%Y-%m-%d").expect("embedded audit date");
    let reference = NaiveDate::parse_from_str(
        text(args, "_current_date", registry.audited_at()),
        "%Y-%m-%d",
    )
    .map_err(|_| invalid("_current_date", "date interne invalide"))?;
    let age_days = (reference - audited).num_days().max(0);
    let stale = age_days > 183 || target != 2026;
    let coverage: Coverage = registry.coverage();
    Ok(response("Actualité fiscale", json!({"summary": format!("Dernier audit réel : **{}** ; registre {} ; statut : **{}**.", registry.audited_at(), registry.version(), if stale {"périmé ou hors année"} else {"à jour"}), "lastAuditDate": registry.audited_at(), "registryVersion": registry.version(), "ageDays": age_days, "staleAfterDays": 183, "isStale": stale, "targetYear": target, "coverage": coverage}), vec!["Date de référence du registre : 1er août 2026.".into()], vec!["Ce contrôle mesure l’âge des données ; il ne remplace pas la revue d’une nouvelle loi de finances.".into()], &[]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn income_tax_rejects_negative_values() {
        let result = invoke(
            "calculer_impot_revenu",
            &json!({"revenu_net_imposable": -1, "situation_famille": "celibataire"}),
        );
        assert!(matches!(result, Err(FiscalError::InvalidArgument { .. })));
    }

    #[test]
    fn zero_income_means_zero_tax() {
        let result = income_tax(
            &json!({"revenu_net_imposable": 0, "situation_famille": "celibataire", "annee": 2026}),
        )
        .unwrap();
        assert_eq!(result.income_tax, 0.0);
    }

    #[test]
    fn tax_is_monotonic_for_single_person() {
        let mut previous = 0.0;
        for income in (0..=500_000).step_by(1_000) {
            let current = income_tax(&json!({"revenu_net_imposable": income, "situation_famille": "celibataire", "annee": 2026})).unwrap().income_tax;
            assert!(current >= previous, "tax decreased at {income}");
            previous = current;
        }
    }

    #[test]
    fn family_quotient_cap_is_applied() {
        let result = income_tax(&json!({"revenu_net_imposable": 250000, "situation_famille": "marie", "nb_enfants": 3, "annee": 2026})).unwrap();
        assert!(result.family_quotient_cap_adjustment > 0.0);
    }

    #[test]
    fn current_pfu_is_31_4_percent() {
        let response = invoke("comparer_pfu_bareme_capital", &json!({"type_revenu": "interets", "montant": 10000, "rni_autres_revenus": 0, "situation_famille": "celibataire"})).unwrap();
        assert_eq!(response.structured_content.result["pfuTax"], 3140.0);
    }

    #[test]
    fn freshness_uses_real_audit_date_and_183_days() {
        let response = invoke("verifier_actualite_fiscale", &json!({"annee_cible": 2026})).unwrap();
        assert_eq!(
            response.structured_content.result["lastAuditDate"],
            "2026-08-01"
        );
        assert_eq!(response.structured_content.result["staleAfterDays"], 183);
        assert_eq!(response.structured_content.result["isStale"], false);
    }

    #[test]
    fn freshness_becomes_stale_after_183_days() {
        let response = invoke(
            "verifier_actualite_fiscale",
            &json!({"annee_cible": 2026, "_current_date": "2027-02-01"}),
        )
        .unwrap();
        assert_eq!(response.structured_content.result["isStale"], true);
        assert!(
            response.structured_content.result["ageDays"]
                .as_i64()
                .unwrap()
                > 183
        );
    }
}
