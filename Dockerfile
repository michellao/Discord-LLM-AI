FROM ubuntu:latest

RUN apt update && apt upgrade -y && \
apt install -y libpq-dev && \
useradd -md /app bot

USER bot

WORKDIR /app

COPY --chown=bot:bot ./target/release/discord_bot_ai .

ENTRYPOINT [ "/app/discord_bot_ai" ]