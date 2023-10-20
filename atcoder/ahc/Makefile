clean:
	@$(eval TOTAL=$(shell find . -name 'Cargo.toml' | wc -l))
	@I=0; for d in *; do if [ -d $$d ] && [ -f $$d/Cargo.toml ]; then cd $$d; I=$$(( I + 1 )); cargo clean; cd ..; progressbar update --total $(TOTAL) --count $$I; fi; done
	@progressbar done
