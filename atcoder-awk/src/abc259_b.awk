{
  theta = $3*atan2(0,-1)/180;
  c = cos(theta);
  s = sin(theta);
  printf "%.9f %.9f",c*$1-s*$2,s*$1+c*$2
}
