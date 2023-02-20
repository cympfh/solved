PROBLEM := 0000

run:
	./tools/target/release/tester cargo run --bin a < ./tools/in/$(PROBLEM).txt | tee /tmp/out | clip
run-release:
	./tools/target/release/tester cargo run --release --bin a < ./tools/in/$(PROBLEM).txt | tee /tmp/out | clip
