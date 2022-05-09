clean:
	for d in *; do if [ -d $$d ]; then echo $$d; cd $$d; [ -f Cargo.toml ] && cargo clean; cd ..; fi; done
