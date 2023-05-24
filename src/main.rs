fn main() {
    let mut queen = [0; 8];
    let board = [[0; 8]; 8];

    'outer: loop {
        // 盤面
        let mut b = board.clone();
        // queenの位置を記録
        for i in 0..queen.len() {
            b[i][queen[i]] = 1;
        }
        // 8体のqueenが条件を満たしているかを確認する
        for i in 0..queen.len() {
            // 縦の確認
            for j in 0..queen.len() {
                if i != j && b[j][queen[i]] != 0 {
                    queen = next(queen);
                    continue 'outer;
                }
            }
            // 斜めの確認
            for sum in 1..14 {
                let mut cnt1 = 0;
                let mut cnt2 = 0;
                for i in 0..8 {
                    if i > sum {
                        break;
                    }
                    let j = sum - i;
                    if j > 7 {
                        continue;
                    }
                    if b[i][j] == 1 {
                        cnt1 += 1;
                    }
                    if b[7 - i][j] == 1 {
                        cnt2 += 1;
                    }
                }
                if cnt1 > 1 {
                    queen = next(queen);
                    continue 'outer;
                }
                if cnt2 > 1 {
                    queen = next(queen);
                    continue 'outer;
                }
            }
        }
        break;
    }
    output(queen);
}

fn next(queen: [usize; 8]) -> [usize; 8] {
    let mut arr = queen;
    arr[0] += 1;
    for j in 0..arr.len() {
        if arr[j] >= 8 {
            arr[j] = 0;
            arr[j + 1] += 1;
        }
    }
    return arr;
}

fn output(queen: [usize; 8]) {
    for i in 0..queen.len() {
        for j in 0..queen.len() {
            if queen[i] == j {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
