func half(x, len) {
  carry = 0;
  for (i = 1; i <= len; ++i) {
    digit = carry * 10 + x[i];
    carry = digit % 2;
    x[i] = int(digit / 2);
  }
}

func greater_than_one(x, len) {
  for (i = 1; i < len; ++i) if (x[i] > 0) return 1;
  if (x[len] > 1) return 1;
  return 0;
}

func dump(x, len) {
  printf "(dump) > ";
  for ( i = 1; i <= len; ++i) printf "%s,",x[i];
  print "";
}

{
  split($1, x, "");
  len = length(x);
  for (i = 1; i <= len; ++i) x[i] = int(x[i]);
  ans = 0
  while (greater_than_one(x, len) == 1) {
    half(x, len);
    ans++;
  }
  print+ans
}
