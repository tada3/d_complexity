use std::cmp::max;
use std::cmp::min;

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
    let c = calc(h, w, &masu);
    println!("{}", c);
}

/**
 * dp1[r1][r2][c1][f]: [r1, r2), [c1, c2)の複雑度がf以下となる最大のc2
 * dp2[c1][c2][r1][f]: [r1, r2), [c1, c2)の複雑度がf以下となる最大のr2
 * メモリ節約のため、fについては現在と一個前の2個分だけ領域を確保する。
 */
fn calc(h: usize, w: usize, masu: &Vec<Vec<char>>) -> i32 {
    let max_f = get_max_f(h, w) + 1; // exclusive
    let mut dp1 = get_4d_vec(2, h, h + 1, w);
    let mut dp2 = get_4d_vec(2, w, w + 1, h);

    //println!("max_f = {}", max_f);

    //println!("Init");
    // Init dp1 A (f=any, r2=r1)
    for f in 0..2 {
        for r1 in 0..h {
            for c1 in 0..w {
                dp1[f][r1][r1][c1] = w as i32;
            }
        }
    }

    // Init dp2 A (f=any, c2 = c1)
    //println!("Init dp2 A");
    for f in 0..2 {
        for c1 in 0..w {
            for r1 in 0..h {
                dp2[f][c1][c1][r1] = h as i32;
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
                dp1[0][r1][r2][c1] = c2 as i32;
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
                dp2[0][c1][c2][r1] = r2 as i32;
                r1 += 1;
            }
        }
    }

    // Init dp1 C (f=0, r2 > r2 + 1)
    //println!("Init dp1 C");
    for r1 in 0..h {
        for r2 in r1 + 2..h + 1 {
            for c1 in 0..w {
                let r2_c1 = dp2[0][c1][c1 + 1][r1];
                let c2 = if r2_c1 < r2 as i32 {
                    c1 as i32
                } else {
                    let c2_prev = dp1[0][r1][r2 - 1][c1];
                    let c2_r2 = dp1[0][r2 - 1][r2][c1]; // Calculated in 'Init dp1 B'
                    min(c2_prev, c2_r2)
                };
                dp1[0][r1][r2][c1] = c2;
                //println!("dp1[0][{}][{}][{}] = {}", r1, r2, c1, c2);
            }
        }
    }

    if dp1[0][0][h][0] == w as i32 {
        //println!("Bingo!");
        return 0;
    }

    // Init dp2 C (f=0, c2 > c1)
    //println!("Init dp2 C");
    for c1 in 0..w {
        for c2 in c1 + 2..w + 1 {
            for r1 in 0..h {
                let c2_r1 = dp1[0][r1][r1 + 1][c1];
                let r2 = if c2_r1 < c2 as i32 {
                    r1 as i32
                } else {
                    let r2_prev = dp2[0][c1][c2 - 1][r1];
                    let r2_c2 = dp2[0][c2 - 1][c2][r1];
                    min(r2_prev, r2_c2)
                };
                dp2[0][c1][c2][r1] = r2;
            }
        }
    }

    if dp2[0][0][w][0] == h as i32 {
        return 0;
    }

    // DP
    //println!("DP");
    for f in 1..max_f {
        for r1 in 0..h {
            for r2 in r1 + 1..h + 1 {
                let mut c2_yoko = 0;
                for c1 in 0..w {
                    // Tate
                    let c2_tate = tate_wari(w, &dp1, r1, r2, c1, f);

                    // Yoko
                    while c2_yoko < w {
                        let r2_next = yoko_wari(h, &dp2, c1, (c2_yoko + 1) as usize, r1, f);
                        if r2_next < r2 as i32 {
                            break;
                        }
                        c2_yoko += 1;
                    }
                    // Choose max
                    dp1[f % 2][r1][r2][c1] = max(c2_tate, c2_yoko as i32);
                }
            }
        }

        if dp1[f % 2][0][h][0] == w as i32 {
            //println!("Bingo!");
            return f as i32;
        }

        for c1 in 0..w {
            for c2 in c1 + 1..w + 1 {
                let mut r2_tate = 0;
                for r1 in 0..h {
                    // Yoko
                    let r2_yoko = yoko_wari(h, &dp2, c1, c2, r1, f);

                    // Tate
                    while r2_tate < h as i32 {
                        let c2_next = tate_wari(w, &dp1, r1, (r2_tate + 1) as usize, c1, f);
                        if c2_next < c2 as i32 {
                            break;
                        }
                        r2_tate += 1;
                    }

                    // Choose max
                    dp2[f % 2][c1][c2][r1] = max(r2_yoko, r2_tate);
                    /*
                    println!(
                        "dp2[{}][{}][{}][{}] = {}, {}",
                        f, c1, c2, r1, r2_yoko, r2_tate
                    );
                    */
                }
            }
        }

        if dp2[f % 2][0][w][0] == h as i32 {
            //println!("Bingo!");
            return f as i32;
        }
    }

    //println!("Fallback");
    return 0;
}

fn get_4d_vec(s1: usize, s2: usize, s3: usize, s4: usize) -> Vec<Vec<Vec<Vec<i32>>>> {
    let mut v1: Vec<Vec<Vec<Vec<i32>>>> = Vec::with_capacity(s1);
    for _i in 0..s1 {
        let mut v2: Vec<Vec<Vec<i32>>> = Vec::with_capacity(s2);
        for _j in 0..s2 {
            let mut v3: Vec<Vec<i32>> = Vec::with_capacity(s3);
            for _k in 0..s3 {
                let v4 = vec![-1; s4];
                v3.push(v4);
            }
            v2.push(v3);
        }
        v1.push(v2);
    }
    return v1;
}

fn tate_wari(
    w: usize,
    dp1: &[Vec<Vec<Vec<i32>>>],
    r1: usize,
    r2: usize,
    c1: usize,
    f: usize,
) -> i32 {
    //println!("XXX tate_wari {}, {}, {}, {}, {}", w, r1, r2, c1, f);
    let c_x = dp1[(f - 1) % 2][r1][r2][c1];
    //println!("XXXX c_x = {}", c_x);
    if c_x == c1 as i32 {
        c1 as i32
    } else if c_x >= w as i32 {
        w as i32
    } else {
        let c_xx = dp1[(f - 1) % 2][r1][r2][c_x as usize];
        if c_xx == c_x {
            c_x
        } else {
            c_xx
        }
    }
}

fn yoko_wari(
    h: usize,
    dp2: &[Vec<Vec<Vec<i32>>>],
    c1: usize,
    c2: usize,
    r1: usize,
    f: usize,
) -> i32 {
    //println!("XXX yoko_wari {}, {}, {}, {}, {}", h, c1, c2, r1, f);
    let r_x = dp2[(f - 1) % 2][c1][c2][r1];
    if r_x == r1 as i32 {
        r1 as i32
    } else if r_x >= h as i32 {
        h as i32
    } else {
        let r_xx = dp2[(f - 1) % 2][c1][c2][r_x as usize];
        if r_xx == r_x {
            r_x
        } else {
            r_xx
        }
    }
}

fn get_max_f(h: usize, w: usize) -> usize {
    let x = (h as f64).log2().ceil() + (w as f64).log2().ceil();
    return x as usize;
}
