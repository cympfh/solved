c++{
  stock = 0
  for (i = 1; i <= NF; ++i) {
    if ($i > NF) {
      stock++;
    } else if (!book[$i]) {
      book[$i] = $i;
    } else {
      stock++;
    }
  }
  n = asort(book, arr);
  last = n;

  for (i = 1; i <= NF; ++i) {
    if (book[i]) {
      book[i] = 0;
    } else if (stock >= 2) {
      stock -= 2;
    } else if (stock >= 1 && book[arr[last]]) {
      stock -= 1;
      book[arr[last]] = 0;
      last -= 1;
    } else if (book[arr[last]] && book[arr[last-1]]) {
      book[arr[last]] = 0;
      book[arr[last-1]] = 0;
      last -= 2;
    } else {
      break;
    }
  }
  print(i - 1);
}
