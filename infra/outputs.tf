output "sns_topic_arn" {
  value       = aws_sns_topic.notifications.arn
  description = "SNS topic ARN for notifications"
}