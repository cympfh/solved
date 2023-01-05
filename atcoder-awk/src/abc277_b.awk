function check(s, c, ok) {
  split(s, c, "");
  ok = 0;
  if (c[1] == "H") ok = 1;
  if (c[1] == "D") ok = 1;
  if (c[1] == "C") ok = 1;
  if (c[1] == "S") ok = 1;
  if (ok == 0) return 0;
  ok = 0;
  if (c[2] == "A") ok = 1;
  if (c[2] == "2") ok = 1;
  if (c[2] == "3") ok = 1;
  if (c[2] == "4") ok = 1;
  if (c[2] == "5") ok = 1;
  if (c[2] == "6") ok = 1;
  if (c[2] == "7") ok = 1;
  if (c[2] == "8") ok = 1;
  if (c[2] == "9") ok = 1;
  if (c[2] == "T") ok = 1;
  if (c[2] == "J") ok = 1;
  if (c[2] == "Q") ok = 1;
  if (c[2] == "K") ok = 1;
  return ok
}

BEGIN {
  ans = "Yes";
}
NR>1 {
  if (check($1) == 0) ans = "No";
  if (set[$1] == 1) ans = "No";
  set[$1] = 1;
}
END {
  print ans;
}
