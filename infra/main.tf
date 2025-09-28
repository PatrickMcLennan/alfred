resource "aws_sns_topic" "notifications" {
  name = "${var.project_name}"
  tags = {
    Project = var.project_name
    Env     = "dev"
  }
}