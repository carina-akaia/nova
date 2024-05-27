# AKAIA Nova

An alternative implementation of NEAR BOS gateway

## Development

### Prerequisites

Install Mise unless it's already installed:

<https://mise.jdx.dev/getting-started.html>

### Installing required tools and dependencies

```sh
mise trust && mise i && cargo install just && just setup
```

### Running development server

```sh
just dev
```

## CI/CD

### Building production bundle

```sh
just build
```
