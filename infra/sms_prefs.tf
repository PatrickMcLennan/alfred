# sms_prefs.tf
resource "aws_sns_sms_preferences" "defaults" {
  monthly_spend_limit                   = 1 # was 5
  default_sms_type                      = "Transactional"
  delivery_status_success_sampling_rate = "100"
}
