# (cargo install --git https://github.com/bram209/leptosfmt.git)

setup:
	(cargo install --locked cargo-make cargo-expand cargo-tree)
	(cargo install cargo-leptos leptosfmt)

vendor:
	(cargo vendor .cache/cargo)

fmt:
	(rustfmt ./src/**/*.rs)
	(leptosfmt ./src)

dev:
	(cargo leptos watch)

build:
	(cargo leptos build --release)

# preview:
#	()

deploy:
	(sudo mkdir -p /akaia/configuration/caddy)
	(sudo cp --update ./system/configuration/caddy/* /akaia/configuration/caddy/Caddyfile)
	(sudo cp --update ./system/configuration/systemd/* /etc/systemd/system/)
	(sudo chown -R akaia:akaia /akaia/configuration)
	(sudo systemctl daemon-reload)
	(sudo systemctl enable --now nova.akaia)
