func min(x,y){return x<y?x:y}
func max(x,y){return x>y?x:y}
func hypot(x, y) {
  return sqrt((x*x) + (y*y));
}
func d(i, j) {
  return hypot(x[i] - x[j], y[i] - y[j]);
}
NR==1{
  n = $1
  k = $2
}
NR==2{
  split($0, a)
}
NR>2{
  x[NR-2] = $1
  y[NR-2] = $2
}
END{
  ans = 0
  for (i = 1; i <= n; ++i) {
    mind = 1000000000.0
    for (j = 1; j <= k; ++j) {
      mind = min(mind, d(i, a[j]));
    }
    ans = max(ans, mind);
  }
  printf "%.9f\n", ans
}
