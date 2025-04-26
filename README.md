# Local chatbot on Discord

## Setup

Requirements:

* llama.cpp or mistral.rs server API or an OpenAI API compatible
* rustup
* Database Postgres


```sh
git clone https://github.com/michellao/Discord-LLM-AI.git
cd Discord-LLM-AI
cargo build --release
```

Create an `.env` from the `.env.example` and edit it to suit your setup.

### Database migration

After completion the .env then install diesel CLI to do database migration

```sh
cd database
cargo install diesel
diesel migration run
```

## Using Docker

You need to configure an environment file to connect a Database.

```sh
docker run --env-file .env ghcr.io/michellao/discord-llm-ai:main
```
