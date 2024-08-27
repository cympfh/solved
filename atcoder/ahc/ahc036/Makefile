evaluate:
	cargo build --release
	: > /tmp/result
	for i in $(shell seq 0 9); do echo Running $$i; cat inputs/seed.$$i | cargo run --release --bin a 1>/dev/null 2>/tmp/r; tail -1 /tmp/r >> /tmp/result; done
	rm /tmp/r
	cat /tmp/result | grep -o '[0-9]*' | jq -s '.|add' | sed 's/.*/sum: &/' >> /tmp/result
	cat /tmp/result
