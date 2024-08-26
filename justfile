# (cargo install --git https://github.com/bram209/leptosfmt.git)

setup:
	(sudo apt-get install pkg-config libssl-dev)
	(cargo install --locked cargo-make cargo-expand cargo-tree)
	(cargo install cargo-leptos leptosfmt)

vendor:
	(cargo vendor .cache/cargo)

fmt:
	(rustfmt ./src/**/*.rs)
	(leptosfmt ./src)

build-framework:
	(cargo make applet_runtime_build)

dev: build-framework
	(cargo make devserver)

build:
	(cargo leptos build --release)

# preview:
#	()

deploy:
	(sudo mkdir -p /akaia/configuration/caddy)
	(sudo cp --update ./src/system/configuration/caddy/* /akaia/configuration/caddy/Caddyfile)
	(sudo cp --update ./src/system/configuration/systemd/* /etc/systemd/system/)
	(sudo chown -R akaia:akaia /akaia/configuration)
	(sudo systemctl daemon-reload)
	(sudo systemctl enable nova.akaia)
	(sudo systemctl restart nova.akaia)
