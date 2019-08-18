use std::cmp::max;
use std::cmp::min;
use std::time::{Duration, Instant};

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let hw = read_vec::<usize>();
    let h = hw[0];
    let w = hw[1];

    let mut masu: Vec<Vec<char>> = Vec::with_capacity(h);
    for _i in 0..h {
        let row = read::<String>().chars().collect::<Vec<char>>();
        masu.push(row);
    }
    let start = Instant::now();
    // std::thread::sleep(Duration::from_millis(1234));
    let c = calc(h, w, &masu);
    let end = start.elapsed();
    println!("{}", c);
    println!("time: {} ns", end.as_secs());
    println!("time: {} ns", end.as_millis());
    println!("time: {} ns", end.as_micros());
    println!("time: {} ns", end.as_nanos());
}

/**
 * dp1[r1][r2][c1][f]: [r1, r2), [c1, c2)の複雑度がf以下となる最大のc2
 * dp2[c1][c2][r1][f]: [r1, r2), [c1, c2)の複雑度がf以下となる最大のr2
 * メモリ節約のため、fについては現在と一個前の2個分だけ領域を確保する。
 */
fn calc(h: usize, w: usize, masu: &Vec<Vec<char>>) -> i32 {
    let max_f = get_max_f(h, w) + 1; // exclusive
    let mut dp1 = vec![vec![vec![vec![0; w]; h+1]; h]; 2];
    let mut dp2 = vec![vec![vec![vec![0; h]; w+1]; w]; 2];
    let h_u8 = h as u8;
    let w_u8 = w as u8;

    //println!("max_f = {}", max_f);

    //println!("Init");
    // Init dp1 A (f=any, r2=r1)
    for f in 0..2 {
        for r1 in 0..h {
            for c1 in 0..w {
                dp1[f][r1][r1][c1] = w_u8;
            }
        }
    }

    // Init dp2 A (f=any, c2 = c1)
    //println!("Init dp2 A");
    for f in 0..2 {
        for c1 in 0..w {
            for r1 in 0..h {
                dp2[f][c1][c1][r1] = h_u8;
            }
        }
    }

    // Init dp1 B (f=0, r2 = r2 + 1)
    for r1 in 0..h {
        let r2 = r1 + 1;
        let mut c1 = 0;
        let mut c2 = c1;
        while c1 < w {
            let v = masu[r1][c1];
            // move c2
            while c2 < w && masu[r1][c2] == v {
                c2 += 1;
            }
            // move c1
            while c1 < c2 {
                dp1[0][r1][r2][c1] = c2 as u8;
                //println!("dp1[0][{}][{}][{}] = {}", r1, r2, c1, c2);
                c1 += 1;
            }
        }
    }

    // Init dp2 B (f=0, c2 = c1 + 1)
    //println!("Init dp2 B");
    for c1 in 0..w {
        let c2 = c1 + 1;
        let mut r1 = 0;
        let mut r2 = r1;
        while r1 < h {
            let v = masu[r1][c1];
            // move r2
            while r2 < h && masu[r2][c1] == v {
                r2 += 1;
            }
            // move r1
            while r1 < r2 {
                dp2[0][c1][c2][r1] = r2 as u8;
                r1 += 1;
            }
        }
    }

    // Init dp1 C (f=0, r2 > r2 + 1)
    //println!("Init dp1 C");
    for r1 in 0..h {
        for r2 in r1 + 2..h + 1 {
            let mut c1 = 0;
            while c1 < w {
                // get c2
                let r2_c1 = dp2[0][c1][c1 + 1][r1] as usize;
                if r2_c1 < r2 {
                    // no one-color region
                    dp1[0][r1][r2][c1] = c1 as u8;
                    c1 += 1;
                } else {
                    // get c2
                    let c2_prev = dp1[0][r1][r2 - 1][c1];
                    let c2_r2 = dp1[0][r2 - 1][r2][c1]; // Calculated in 'Init dp1 B'
                    let c2_u8 = min(c2_prev, c2_r2);
                    let c2 = c2_u8 as usize;
                    // move c1
                    while c1 < c2 as usize {
                        dp1[0][r1][r2][c1] = c2_u8;
                        c1 += 1;
                    }
                }
            }
        }
    }

    if dp1[0][0][h][0] == w_u8 {
        //println!("Bingo!");
        return 0;
    }

    // Init dp2 C (f=0, c2 > c1)
    //println!("Init dp2 C");
    for c1 in 0..w {
        for c2 in c1 + 2..w + 1 {
            let mut r1 = 0;
            while r1 < h {
                let c2_r1 = dp1[0][r1][r1 + 1][c1] as usize;
                if c2_r1 < c2 {
                    dp2[0][c1][c2][r1] = r1 as u8;
                    r1 += 1;                   
                } else {
                    let r2_prev = dp2[0][c1][c2 - 1][r1];
                    let r2_c2 = dp2[0][c2 - 1][c2][r1];
                    let r2_u8 = min(r2_prev, r2_c2);
                    let r2 = r2_u8 as usize;
                    while r1 < r2 {
                        dp2[0][c1][c2][r1] = r2_u8;
                        r1 += 1;
                    }
                }
            }
        }
    }

    if dp2[0][0][w][0] == h_u8 {
        return 0;
    }

    // DP
    //println!("DP");
    for f in 1..max_f {
        let f_cur = f %2;
        let f_prev = (f-1) % 2;
        for r1 in 0..h {
            for r2 in r1 + 1..h + 1 {
                let mut c1 = 0;
                let mut c2_yoko = 0;
                while c1 < w {
                    // Tate
                    let c2_tate = tate_wari(w, &dp1, r1, r2, c1, f_prev);

                    // Yoko
                    while c2_yoko < w && yoko_wari(h, &dp2, c1, c2_yoko + 1, r1, f_prev) >= r2 {
                        c2_yoko += 1;
                    }

                    // Choose max
                    let c2 = max(c2_tate, c2_yoko);
                    dp1[f_cur][r1][r2][c1] = c2 as u8;
                    c1 += 1;

                    if c2 == w {
                        // if c2 is w, for all cx > c1, dp1[f][r1][r2][cx] = w.
                        while c1 < w {
                            dp1[f_cur][r1][r2][c1] = w_u8;
                            c1 += 1;
                        }
                    }
                }
            }
        }

        if dp1[f_cur][0][h][0] == w_u8 {
            //println!("Bingo!");
            return f as i32;
        }

        for c1 in 0..w {
            for c2 in c1 + 1..w + 1 {
                let mut r1 = 0;
                let mut r2_tate = 0;
                while r1 < h {
                    // Yoko
                    let r2_yoko = yoko_wari(h, &dp2, c1, c2, r1, f_prev);

                    // Tate
                    while r2_tate < h && tate_wari(w, &dp1, r1, r2_tate + 1, c1, f_prev) >= c2 {
                        r2_tate += 1;
                    }

                    // Choose max
                    let r2 = max(r2_yoko, r2_tate);
                    dp2[f_cur][c1][c2][r1] = r2 as u8;
                    r1 += 1;

                    if r2 == h {
                        while r1 < h {
                            dp2[f_cur][c1][c2][r1] = h_u8;
                            r1 += 1;
                        }
                    }
                }
            }
        }

        if dp2[f_cur][0][w][0] == h_u8 {
            //println!("Bingo!");
            return f as i32;
        }
    }

    //println!("Fallback");
    return 0;
}

fn tate_wari(
    w: usize,
    dp1: &[Vec<Vec<Vec<u8>>>],
    r1: usize,
    r2: usize,
    c1: usize,
    f_prev: usize,
) -> usize {
    let c_x = dp1[f_prev][r1][r2][c1] as usize;
    if c_x == c1 {
        c1
    } else if c_x >= w {
        w
    } else {
        dp1[f_prev][r1][r2][c_x] as usize
    }
}

fn yoko_wari(
    h: usize,
    dp2: &[Vec<Vec<Vec<u8>>>],
    c1: usize,
    c2: usize,
    r1: usize,
    f_prev: usize,
) -> usize {
    let r_x = dp2[f_prev][c1][c2][r1] as usize;
    if r_x == r1 {
        r1
    } else if r_x >= h {
        h
    } else {
        dp2[f_prev][c1][c2][r_x] as usize
    }
}

fn get_max_f(h: usize, w: usize) -> usize {
    let x = (h as f64).log2().ceil() + (w as f64).log2().ceil();
    return x as usize;
}
