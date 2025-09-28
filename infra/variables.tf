variable "project_name" {
  type        = string
  description = "Project prefix for resource names"
  default     = "alfred-dispatch-movie-added"
}

variable "aws_region" {
  type        = string
  default     = "ca-central-1"
  description = "AWS region for all resources"
}