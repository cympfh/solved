test:
	for f in ./input/*.txt; do echo $$f; cat $$f | cargo run --bin a --release >/dev/null; done

localscore:
	( for f in input/01*.txt; do cat $$f | cargo run --bin a --release --quiet >/dev/null; done ) 2>/tmp/scores
	cat /tmp/scores | grep -v update | grep -o '[0-9][0-9]*$$' | jq -s 'add'
