{
  k = $2
  split($1, x, "");
  carry = 0

  if (length(x) <= k - 1) {
    print 0
    exit
  }

  for (i = length(x); i > length(x) - k; --i) {
    digit = int(x[i]) + carry;
    if (digit >= 5) {
      carry = 1;
    } else {
      carry = 0;
    }
    x[i] = 0;
  }
  for (i = length(x) - k; i >= 1; --i) {
    digit = int(x[i]) + carry;
    if (digit < 10) {
      x[i] = digit;
      carry = 0;
    } else {
      x[i] = digit % 10;
      carry = 1;
    }
  }
  if (carry > 0) {
    printf "1";
    for (i = 1; i <= length(x); ++i) printf "%s", x[i];
    print ""
  } else {
    zero_leading = 1;
    for (i = 1; i <= length(x); ++i) {
      if (zero_leading == 1 && x[i] == 0) continue;
      zero_leading = 0;
      printf "%s", x[i];
    }
    if (zero_leading) printf "0";
    print ""
  }


  exit
}
