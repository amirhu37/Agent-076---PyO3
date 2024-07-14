build:
	cargo build

develop:
	maturin develop
	python tests/test.py 

fmt:
	cargo fmt

run:
	cargo run

test:
	python tests/test.py 

env : 
	new_env\Scripts\activate

