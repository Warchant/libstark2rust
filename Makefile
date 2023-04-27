help:
	@echo Run `make init -j10` once, it will fetch and compile libSTARK.
	@echo Subsequently, you can run `make build` to build final binary.
	@echo Run `make run` to execute the binary. It should print true\ntrue\n

.ONESHELL:
init:
	@git submodule update --init
	@echo Patching libSTARK...
	@(
		cd libSTARK
		git clean -fdx .
		git reset --hard
		git apply ../1.patch
		$(MAKE)
	)

build:
	cargo build

run:
	cargo run

.ONESHELL:
clean:
	cargo clean
	(
		cd libSTARK
		git clean -fdx .
	)
