variable "db_user" {
  description = "Postgres superuser name"
  type        = string
  default     = "root"
}

variable "db_password" {
  description = "Postgres superuser password"
  type        = string
  default     = "root"
  sensitive   = true
}

variable "db_name" {
  description = "Default database created on init"
  type        = string
  default     = "backend"
}

variable "host_port" {
  description = "Host port mapped to the container's 5432"
  type        = number
  default     = 5432
}
