{x[NR-1]=$1;y[NR-1]=$2}
func cross(x1,y1,x2,y2){return x1*y2-x2*y1}
END{
  ans = "Yes";
  for (i = 0; i < 4; ++i) {
    i1 = (i + 1) % 4;
    i2 = (i + 2) % 4;
    c = cross(x[i1] - x[i], y[i1] - y[i], x[i2] - x[i1], y[i2] - y[i1])
    if (c < 0) ans = "No";
  }
  print ans
}
