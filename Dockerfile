FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && \
apt-get install -y libpq-dev && \
useradd -md /app bot && \
apt-get clean

USER bot

WORKDIR /app

COPY --chown=bot:bot --chmod=700 ./target/release/discord_bot_ai .

ENTRYPOINT [ "/app/discord_bot_ai" ]