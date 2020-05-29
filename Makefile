.PHONY: dev

dev:
	rm -Rf .deno_plugins/deno_mdns*
	cargo build
	RUST_BACKTRACE=full DENO_MDNS_DEBUG=1 DENO_MDNS_PLUGIN_BASE=http://localhost:5000 deno run --reload --unstable -A ./mod.ts