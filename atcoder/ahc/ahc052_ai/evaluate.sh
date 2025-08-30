#!/bin/bash

# AHC052 評価システム
# 全100個の入力ファイルでソルバーを実行し、平均スコアを計算

set -e

# 設定
INPUTS_DIR="inputs"
SOLVER_BIN="target/release/a"
TEMP_DIR="temp_eval"

echo "Evaluation:" >&2

# リリースビルド
echo "Building solver in release mode..." >&2
cargo build --release --quiet

# 一時ディレクトリ作成
mkdir -p $TEMP_DIR

# 変数初期化
total_score=0
completed_count=0
failed_count=0
perfect_count=0

# 入力ファイルを番号順にソート
for input_file in $(ls $INPUTS_DIR/*.txt | sort -V); do
    filename=$(basename "$input_file")
    case_num=${filename%.txt}
    
    echo -n "Case $case_num: " >&2
    
    # ソルバー実行（標準エラー出力を一時ファイルに保存）
    if timeout 3s ./$SOLVER_BIN < "$input_file" > "$TEMP_DIR/output_$case_num.txt" 2> "$TEMP_DIR/stderr_$case_num.txt"; then
        # 標準エラー出力からスコアを抽出
        if grep -q "FINAL_SCORE:" "$TEMP_DIR/stderr_$case_num.txt"; then
            score=$(grep "FINAL_SCORE:" "$TEMP_DIR/stderr_$case_num.txt" | tail -1 | sed 's/FINAL_SCORE: \([0-9]\+\)/\1/')
            if [[ "$score" =~ ^[0-9]+$ ]]; then
                total_score=$((total_score + score))
                completed_count=$((completed_count + 1))
                if [ $score -ge 900 ]; then
                    perfect_count=$((perfect_count + 1))
                fi
                echo "Score: $score" >&2
            else
                echo "スコア解析失敗" >&2
                failed_count=$((failed_count + 1))
            fi
        else
            echo "スコア出力なし" >&2
            failed_count=$((failed_count + 1))
        fi
    else
        echo "実行失敗/タイムアウト" >&2
        failed_count=$((failed_count + 1))
    fi
done

# 一時ファイル削除
rm -rf $TEMP_DIR

echo ""
echo "=== 評価結果 ==="
echo "成功: $completed_count / 100"
echo "失敗: $failed_count / 100"
echo "完璧: $perfect_count / 100"

if [ $completed_count -gt 0 ]; then
    average_score=$((total_score / completed_count))
    echo "合計スコア: $total_score"
    echo "平均スコア: $average_score"
    
    # 成功率計算
    success_rate=$((completed_count * 100 / 100))
    echo "成功率: $success_rate%"
else
    echo "計算可能なスコアがありません"
fi
