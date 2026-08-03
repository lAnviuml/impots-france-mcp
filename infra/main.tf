terraform {
  required_version = "= 1.12.5"

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "= 5.22.0"
    }
  }

  backend "s3" {
    key                         = "impots-france-mcp/access.tfstate"
    region                      = "auto"
    skip_credentials_validation = true
    skip_region_validation      = true
    skip_requesting_account_id  = true
    skip_s3_checksum            = true
    use_path_style              = true
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

locals {
  applications = {
    for environment in var.enabled_environments : environment => var.application_urls[environment]
  }
}

resource "cloudflare_zero_trust_access_application" "mcp" {
  for_each = local.applications

  account_id       = var.cloudflare_account_id
  name             = "Impôts France MCP — ${each.key}"
  type             = "mcp"
  session_duration = "24h"

  destinations = [{
    type = "public"
    # Cloudflare Access requires MCP applications to target a hostname root.
    # The MCP transport itself remains available at the /mcp URL below.
    uri = trimsuffix(each.value, "/mcp")
  }]

  oauth_configuration = {
    enabled = true
    dynamic_client_registration = {
      enabled                = true
      allow_any_on_localhost = true
      allow_any_on_loopback  = true
      allowed_uris           = var.remote_redirect_uris
    }
    grant = {
      access_token_lifetime = "15m"
      session_duration      = "336h"
    }
  }

  policies = [{
    name       = "Compte autorise"
    decision   = "allow"
    precedence = 1
    include = [{
      email = {
        email = var.authorized_email
      }
    }]
  }]
}

variable "cloudflare_api_token" {
  type        = string
  description = "Jeton Cloudflare limité à Access Apps/Policies Write."
  sensitive   = true
}

variable "cloudflare_account_id" {
  type        = string
  description = "Identifiant du compte Cloudflare."
}

variable "authorized_email" {
  type        = string
  description = "Adresse exacte du seul compte autorisé par Cloudflare Access."
  sensitive   = true
}

variable "application_urls" {
  type        = map(string)
  description = "URLs MCP HTTPS par environnement, chemin /mcp inclus."

  validation {
    condition = alltrue([
      for environment, uri in var.application_urls :
      contains(["staging", "production"], environment) && startswith(uri, "https://") && endswith(uri, "/mcp")
    ])
    error_message = "Chaque cle doit etre staging ou production et chaque URL doit utiliser HTTPS avec le chemin /mcp."
  }
}

variable "enabled_environments" {
  type        = set(string)
  description = "Environnements Access gérés par cet apply. Après la bascule, conserver staging et production pour éviter une destruction involontaire."
  default     = ["staging", "production"]

  validation {
    condition = alltrue([
      for environment in var.enabled_environments : contains(["staging", "production"], environment)
    ])
    error_message = "Les seuls environnements valides sont staging et production."
  }
}

variable "remote_redirect_uris" {
  type        = list(string)
  description = "Callbacks HTTPS explicitement autorisés pour les clients MCP distants."
  default     = []

  validation {
    condition     = alltrue([for uri in var.remote_redirect_uris : startswith(uri, "https://")])
    error_message = "Chaque callback distant doit utiliser HTTPS."
  }
}

output "access_audiences" {
  description = "Audience exacte à configurer dans ACCESS_AUD pour chaque Worker."
  value       = { for environment, application in cloudflare_zero_trust_access_application.mcp : environment => application.aud }
  sensitive   = true
}

output "mcp_urls" {
  value = local.applications
}
