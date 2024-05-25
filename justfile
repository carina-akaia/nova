# (cargo install --git https://github.com/bram209/leptosfmt.git)

setup:
	(cargo install cargo-make cargo-leptos leptosfmt)

vendor:
	(cargo vendor .cache/cargo)

dev:
	(cargo leptos watch)

build:
	(cargo leptos build --release)

# preview:
#	()
