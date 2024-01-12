build:
	cargo build

shellspec:
	@shellspec -f tap specs

tests: shellspec
