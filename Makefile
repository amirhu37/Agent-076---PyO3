build:
	cargo build

develop:
	maturin develop
	

fmt:
	cargo fmt

debug:
	maturin develop
	python tests/test.py 


test:
	python tests/test.py 

env : 
	new_env\Scripts\activate

