CARGO=RUSTFLAGS='-D warnings -A deprecated -Clink-arg=-Wl,--allow-multiple-definition' cargo

.PHONY: debug release test test-release bench fmt clean clippy

debug:
	$(CARGO) build --all

release:
	$(CARGO) build -p executor  --release
	$(CARGO) build -p executor_plugin  --release --target wasm32-unknown-unknown

test:
	RUST_BACKTRACE=full $(CARGO) test --all 2>&1

test-release:
	RUST_BACKTRACE=full $(CARGO) test --release --all

bench:
	-rm target/bench.log
	cargo bench --all --no-run |tee target/bench.log
	cargo bench --all --jobs 1 |tee -a target/bench.log

fmt:
	cargo fmt --all -- --check

clean:
	rm -rf target/debug/
	rm -rf target/release/

clippy:
	$(CARGO) clippy --all
