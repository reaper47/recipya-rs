# Pod deployment

This guide show how to run Recipya in a Pod.
Please consult the quadlets subdirectory if you want to run Recipya using systemd instead.

## Deployment

Please refer to the [installation guide](../../docs/installation.md#pod) for details.

## Pod management

Stop the pod: 
```bash
podman pod stop recipya-rs
```

Start the pod: 
```bash
podman pod start recipya-rs
```

Restart the pod: 
```bash
podman pod restart recipya-rs
```

Fetch logs for pod: 
```bash
podman pod logs recipya-rs
```

Display a live stream of resource usage statistics for the containers in the pod: 
```bash
podman pod stats recipya-rs
```

Remove the pod: 
```bash
podman pod rm recipya-rs
```
