NR==1{
  n = $1
  m = $2
}
NR >= 2 && NR <= 1 + n {
  len = $1
  for (i = 2; i <= 1 + len; ++i) {
    key = (NR - 1) "," (i - 1)
    a[key] = $i
  }
}
NR > 1 + n {
  key = $1 "," $2
  print a[key]
}
