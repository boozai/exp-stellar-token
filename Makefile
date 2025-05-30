default: build

build:
	stellar contract build --out-dir opt/
	@ls -l opt/*.wasm

deploy: build
	stellar contract deploy --wasm opt/token.wasm --alias contract

mint:
	stellar contract invoke --id contract -- mint --to me --amount 1
