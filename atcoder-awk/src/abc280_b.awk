# 入力から N と S の配列を読み込む
NR == 1 {
    N = $1
}
NR == 2 {
    for (i = 1; i <= N; i++) {
        S[i] = $i
    }
}

# A の配列を定義する
{
    A[1] = S[1]
    for (i = 2; i <= N; i++) {
        A[i] = S[i] - S[i - 1]
    }
}

# A の配列を出力する
END {
    for (i = 1; i <= N; i++) {
        printf("%d ", A[i])
    }
}
