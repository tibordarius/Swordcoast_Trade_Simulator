.PHONY: reference preflight release-check rust-test static
reference:
	python tools/run_reference_suite.py

preflight:
	python tools/preflight.py

release-check:
	python tools/release_check.py

rust-test:
	cd code && cargo test --workspace --all-targets

static:
	python -m http.server 8080 --directory code/web
