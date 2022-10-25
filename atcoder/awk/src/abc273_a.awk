func f(x) {
  if (x <= 1) return 1;
  return x * f(x-1);
}
{print f($1)}
