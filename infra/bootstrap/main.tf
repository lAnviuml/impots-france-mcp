terraform {
  required_version = "= 1.12.5"

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "= 5.22.0"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

resource "cloudflare_r2_bucket" "tofu_state" {
  account_id    = var.cloudflare_account_id
  name          = var.state_bucket_name
  jurisdiction  = "eu"
  location      = "weur"
  storage_class = "Standard"
}

variable "cloudflare_api_token" {
  type        = string
  description = "Jeton Cloudflare avec Workers R2 Storage Write."
  sensitive   = true
}

variable "cloudflare_account_id" {
  type        = string
  description = "Identifiant du compte Cloudflare."
}

variable "state_bucket_name" {
  type        = string
  description = "Nom globalement unique du bucket R2 pour l’état OpenTofu."
  default     = "impots-france-mcp-tofu-state"
}

output "state_bucket_name" {
  value = cloudflare_r2_bucket.tofu_state.name
}
