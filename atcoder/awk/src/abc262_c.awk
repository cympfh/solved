c++{
  ans = 0
  a = 0
  for (i = 1; i <= NF; ++i) if (i == $i) ++a;
  ans += a * (a - 1) / 2;
  for (i = 1; i <= NF; ++i) if ($i > i && $($i) == i) ++ans;
  print+ans
}
