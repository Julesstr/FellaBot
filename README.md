# DotA 2 Inhouse Matchmaking Bot
To run and build the latest version
```shell
docker compose build  && docker compose up
```

Configure your `DISCORD_TOKEN` in the `docker-compose.yml`. Adjust the `environment` section so that the `DISCORD_TOKEN` is not commented and place your value.
DO NOT COMMIT YOUR TOKEN!
```yaml
    environment:
      # Add any environment variables your application needs
      - NODE_ENV=production
    #      - SURVEY_URL=
    #      - APP_ID=
    #      - DISCORD_TOKEN=
    #      - PUBLIC_KEY=
```