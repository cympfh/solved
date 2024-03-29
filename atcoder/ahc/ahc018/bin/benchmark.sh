#!/bin/bash

TESTER=./tools/target/release/tester
BIN=./target/release/a

cargo build --release --bin a
touch result.log
cp /dev/null /tmp/result.log

info() {
  cat | tee -a /tmp/result.log | tee -a result.log
}

echo | info
date | info
git hash | info
echo | info

for i in $(seq 80 99); do
  p=$(printf "%04d" $i)
  printf "Problem = $p\t" | info
  $TESTER $BIN < "./tools/in/$p.txt" >/dev/null 2>/tmp/result
  cat /tmp/result | info
done

(
  echo -n "Sum = "
  cat /tmp/result.log | grep 'Total Cost' | awk '{print $NF}' | jq -s add
) | tee -a result.log
