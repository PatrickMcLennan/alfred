# alfred

## my personal assistant

I have a NAS that doubles as a home server. Aside from serving 60TB+ of media, it also runs several docker containers, such as a plex server, homeassistant, pihole, overseerr, etc. alfred is a monorepo of services and utilities that gives me greater observability and insight into each of these services, allowing me to perform more bespoke home automations.

alfred integrates with AWS for specific features (SMS notifications, etc), and is otherwise a Rust monorepo utilizing cargo workspaces.

## Services

### 📲 Notify New Movie

> _**Requires**: Plex_

The library of movies on my Plex server largely manages itself, new movies are added and old movies are removed programatically based on a variety of factors. This service sends me an SMS message every time a new movie is added.

Rust | Terraform | AWS (+ SNS)

### Movie Recommendation engine

> _**Requires**: Plex, Tautulli_

Stores and batches recently watched movies via Tautulli webhooks. Sends batches to OpenAI API on a schedule to receive customized movie recommendations.

Rust | OpenAI | TMDB API

## Running

Fill out the `.env` (see `.env.example`) and run

```bash
docker-compose up -d;
```

## Developing locally

Development is done via running Rust code locally and using [AWS's Sandbox](https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html) functionality.

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

Visit the AWS SNS Dashboard to get the `$NOTIFY_NEW_MOVIE_SNS_ARN` from your new Topic. Then run:

```bash
cargo run -p notify_new_movie -- \
  --topic-arn $NOTIFY_NEW_MOVIE_SNS_ARN
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
