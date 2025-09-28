### Set up AWS

```bash
## You'll need the aws cli and terraform first
brew install terraform awscli;
aws configure; ## Get info from AWS Dashboard

cd ./infra;
terraform init;
terraform plan -out=tfplan;
terraform apply "tfplan";
```

### Running locally

Local dev involves running `dispatch` locally and using [AWS's SMS Sandbox](https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html) to test e2e. [Scaffold terraform and AWS](#set-up-aws), then visit the AWS SNS Dashboard to get the `$SNS_ARN` from your new Topic. Then run:

```bash
cargo run -p dispatch -- \
  --topic-arn $SNS_ARN
```

### Deploy

#### 1. Run tests and compile for prod

```bash
cargo test && cargo build --release;
```

#### 2. (if applicable) Implement Terraform changes

```bash
terraform plan -out=tfplan;
terraform apply "tfplan";
```
