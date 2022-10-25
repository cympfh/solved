{
  a = $1
  b = $2
  c = 0
  if ((a % 2) + (b % 2) > 0) c += 1;
  a = int(a/2);
  b = int(b/2);
  if ((a % 2) + (b % 2) > 0) c += 2;
  a = int(a/2);
  b = int(b/2);
  if ((a % 2) + (b % 2) > 0) c += 4;
  print c
}
