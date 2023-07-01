reproduce:
	rustc -O --crate-type lib -C target-feature=+zba -C target-feature=+zbb src/lib.rs && objdump -d liblib.rlib |less -p interesting
