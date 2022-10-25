NR == 1 {
  n = $1
  t = $3
  cur = 1
  ok = 1
}
NR == 2 {
  split($0, a)
}
NR > 2 {
  x = $1
  y = $2
  for (;cur < x; ++cur) {
    t -= a[cur];
    if (t <= 0) {
      ok = 0
    }
  }
  t += y
}
END {
  for (; cur < n; ++cur) {
    t -= a[cur];
    if (t <= 0) {
      ok = 0
    }
  }
  if (t < 0) ok = 0;
  print(ok ? "Yes" : "No")
}
