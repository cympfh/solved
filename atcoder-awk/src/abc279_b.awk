# S と T を入力
{ S = $0; getline T; }

# 文字列 T が文字列 S の部分文字列かどうかを判定
{
  if (index(S, T) > 0) {
    print "Yes"
  } else {
    print "No"
  }
}
