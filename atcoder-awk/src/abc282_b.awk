NR==1{
  N=$1
  M=$2
}
NR>1{
  split($0,line,"");
  i = NR - 2;
  for (j = 0; j < M; ++j) {
    a[i * M + j] = line[j+1];
    # print("a", (i*M+j), a[i*M+j]);
  }
}
END {
  for (i1 = 0; i1 < N; ++i1) {
    for (i2 = i1 + 1; i2 < N; ++i2) {
      ok = 1;
      for (j = 0; j < M; ++j) {
        if (a[i1*M+j] == "x" && a[i2*M+j] == "x") ok = 0;
      }
      if (ok == 1) {
        # print(i1, i2);
        ans++;
      }
    }
  }
  print+ans;
}
