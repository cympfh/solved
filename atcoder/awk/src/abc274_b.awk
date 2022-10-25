w{
  split($1, a, "");
  for (i = 1; i <= w; ++i) {
    if (a[i] == "#") ans[i]++;
  }
}
!w{w=$2}
END {
  for (i = 1; i <= w; ++i) {
    print(+ans[i]);
  }
}
