n{
  g[1] = 0;
  for (i = 1; i <= NF; ++i) {
    g[i*2] = g[$i] + 1;
    g[i*2+1] = g[$i] + 1;
  }
  for (i = 1; i <= 2*n+1; ++i) print+g[i];
}
!n{n=$1}
