# Runbook

The service is running on a basic Digital Ocean Ubuntu droplet.
The process itself runs in a dedicated `tmux` session named `badgesize`.

## Routines

### Start the service

```sh
sudo cargo run --release -- --port=80 --logpath=/var/log
```

### Update the service

```sh
# hit `ctrl+c` to stop the service

# update using git
git pull

# start the service
sudo cargo run --release -- --port=80 --logpath=/var/log
```

## Observability

## Status

The service is continuously tested every 10 minutes with [Better Uptime](https://betteruptime.com/).
When down an alert is sent via email and it updated automatically the status page located at: https://status.badgesize.io.

## Inspecting traces

Traces are stored in a local log file located at `/var/log/badgesize.log`.
The log file is rotated every week via `logrotate` with a 1 year rentention period.

To inspect the traces live:

```sh
tail -f /var/log/badgesize.log
```
