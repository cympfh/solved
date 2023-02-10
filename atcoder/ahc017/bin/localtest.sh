#!/bin/bash

UPDATE=1
if [ $NOUPDATE ]; then
  UPDATE=0
else
  UPDATE=1
  echo >> result.log
fi

log() {
  if [ $UPDATE = 1 ]; then
    cat | tee -a result.log
  else
    cat
  fi
}

date | log
git log -n1 | log
for i in $(seq 0 9); do
  seed=$(printf "%04d" $i)
  printf "seed = ${seed}\t" | log
  cargo run --release --bin a < "tools/in/${seed}.txt" > /tmp/out 2>/dev/null
  ./tools/target/release/vis "tools/in/${seed}.txt" /tmp/out | awk '{print $3, log($3)/log(10)}' | log
done
