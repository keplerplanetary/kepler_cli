run:
	RUST_LOG=info cargo run -- -f example.toml

clean: 
	rm -r export_files
