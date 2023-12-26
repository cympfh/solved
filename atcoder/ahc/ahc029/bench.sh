#!/bin/bash

(
  for i in $(seq 0 99); do
    ID=$(printf "%04d" $i)
    ./tools/target/release/tester cargo run --bin a --release < ./tools/in/$ID.txt >/dev/null 2>/tmp/out
    echo -n "$ID: "
    tail -n1 /tmp/out
  done
) > /tmp/result

(
  echo -n "Sum: "
  cat /tmp/result | grep -o '[0-9]*$' | jq -s 'add'
  cat /tmp/result
) | tee /tmp/result2
