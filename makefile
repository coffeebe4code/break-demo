WTGT=./break-wasm/pkg

.PHONY: cwgame
cwgame:
	cargo watch -x "run --bin break-game"

.PHONY: wasm-dev
wasm-dev:
	wasm-pack build break-wasm --target web --dev
	cd break-wasm && python3 -m http.server

.PHONY: clean
clean:
	rm -rf $(TARGET)
	rm -rf $(WTGT)
