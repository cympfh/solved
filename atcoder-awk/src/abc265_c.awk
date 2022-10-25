NR == 1 {
  h = $1
  w = $2
}
NR > 1 {
  i = NR - 1;
  split($0, line, "");
  for (j = 1; j <= w; ++j)
    data[i * 600 + j] = line[j];
}
END {
  i = 1;
  j = 1;
  timestep = 0;
  failed = 0;
  for (;!failed;) {
    timestep++;
    if (timestep > h * w + 100) {
      failed = 1;
    }
    d = data[i * 600 + j];
    if (d == "U" && i > 1)  i--;
    else if (d == "D" && i < h) i++;
    else if (d == "L" && j > 1) j--;
    else if (d == "R" && j < w) j++;
    else break;
  }
  if (failed) { print -1; }
  else {
    print i, j
  }
}
