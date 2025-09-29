# alfred

## tl;dr -- my personal assistant

I have a NAS that doubles as a home server. Aside from serving 60TB+ of media, it also runs several docker containers, such as a plex server, homeassistant, pihole, overseerr, etc. alfred is a monorepo of services I run that allow me to inject custom eventing and logic across each of these docker containers, allowing for bespoke home automation triggers and .

alfred integrates with AWS for specific features (SMS notifications, etc), and is otherwise a Rust monorepo utilizing cargo workspaces.

## Services

### 💹 1. Dashboard

A very simple dashboard and overview of alfreds performance.

### 🧠 2. Movie Recommendation engine

> _**Requires**: Plex, Tautulli_, _Plex Meta Manager_

Stores and batches recently watched movies via Tautulli webhooks. Sends batches to OpenAI API on a schedule to receive customized movie recommendations, storing them in a local NDJSON file (for now).

Rust | OpenAI | TMDB API

### 📲 3. Notify New Movie

> _**Requires**: Plex_

The library of movies on my Plex server largely manages itself, new movies are added and old movies are removed programatically based on a variety of factors. This service sends me an SMS message every time a new movie is added.

Rust | Terraform | AWS (+ SNS)

## Running

1. Run the terraform code to set up your AWS env.

```bash
brew install terraform awscli;
aws configure; ## Get info from AWS Dashboard

cd ./infra;
terraform init;
terraform plan -out=tfplan;
terraform apply "tfplan";
```

2. Fill out the `.env` (see `.env.example`, you'll need the ARN created with the above terraform commands)
3. Start your docker containers

```bash
docker-compose up -d;
```

That's it! 🎉
