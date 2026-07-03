output "database_url" {
  description = "Connection string for sqlx (DATABASE_URL)"
  value       = "postgres://${var.db_user}:${var.db_password}@localhost:${var.host_port}/${var.db_name}"
  sensitive   = true
}
