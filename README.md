# alfred

I have a NAS that also acts as a home server, running docker containers for things like a plex server, homeassistant, pihole, overseerr, etc. alfred is a monorepo of services and utilities that orhchestrates bespoke home automation behaviour based on these services and their usage.

alfred integrates with AWS for specific features (SMS notifications, etc), and is otherwise a Rust monorepo utilizing cargo workspaces.

## Services

### 📲 Notify New Movie

The library of movies on my Plex server largely manages itself, new movies are added and old movies are removed automatically based on a variety of factors. This service sends me an SMS message when a new movie is added.

Rust | Terraform | AWS (+ SNS)

## Developing locally

There is no "true" local development for alfred. Development is done via running Rust code locally and using [AWS's SMS Sandbox](https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html) functionality.

### 1. Set up AWS

```bash
brew install terraform awscli;
aws configure; ## Get info from AWS Dashboard

cd ./infra;
terraform init;
terraform plan -out=tfplan;
terraform apply "tfplan";
```

### 2. Running locally

Visit the AWS SNS Dashboard to get the `$SNS_ARN` from your new Topic. Then run:

```bash
cargo run -p notify_new_movie -- \
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
