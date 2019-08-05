use std::cmp;

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
    let c = calc3(h, w, &masu);
    println!("{}", c);
}

/**
 * dp1[r1][r2][c1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のc2
 * dp2[c1][c2][r1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のr2
 * メモリ節約のため、fの分は現在と一個前の2個分だけ領域を確保する。
 */
fn calc3(h: usize, w: usize, masu: &Vec<Vec<char>>) -> i32 {
    let max_f = get_max_f(h, w) + 1; // exclusive
    let mut dp1 = get_4d_vec(h, h, w, 2);
    let mut dp2 = get_4d_vec(w, w, h, 2);

    // Initialization (f = 0)
    for r1 in 0..h {
        for r2 in r1..h {
            for c1 in 0..w {
                let mut c2 = c1 as i32;
                if r2 == r1 {
                    if c1 < w - 1 {
                        let v = masu[r1][c1];
                        if c1 > 0 && v == masu[r1][c1 - 1] {
                            c2 = dp1[r1][r2][c1 - 1][0];
                        } else {
                            while c2 < (w - 1) as i32 && masu[r1][(c2 + 1) as usize] == v {
                                c2 += 1;
                            }
                        }
                    }
                } else {
                    let c2_prev = dp1[r1][r2 - 1][c1][0];
                    let v_prev = masu[r2 - 1][c1];
                    if c2_prev < 0 || masu[r2][c1] != v_prev {
                        c2 = -1;
                    } else {
                        let c2_prev = dp1[r1][r2 - 1][c1][0];
                        while c2 + 1 <= c2_prev && masu[r2][(c2 + 1) as usize] == v_prev {
                            c2 += 1;
                        }
                    }
                }
                dp1[r1][r2][c1][0] = c2;
                //println!("dp1[{}][{}][{}][{}] = {}", r1, r2, c1, 0, c2);
            }
        }
    }

    if dp1[0][h - 1][0][0] == (w - 1) as i32 {
        return 0;
    }

    for c1 in 0..w {
        for c2 in c1..w {
            for r1 in 0..h {
                let mut r2 = r1 as i32;
                if c2 == c1 {
                    if r1 < h - 1 {
                        let v = masu[r1][c1];
                        if r1 > 0 && v == masu[r1 - 1][c1] {
                            r2 = dp2[c1][c2][r1 - 1][0];
                        } else {
                            while r2 < (h - 1) as i32 && masu[(r2 + 1) as usize][c1] == v {
                                r2 += 1;
                            }
                        }
                    }
                } else {
                    let r2_prev = dp2[c1][c2 - 1][r1][0];
                    let v_prev = masu[r1][c2 - 1];
                    if r2_prev < 0 || masu[r1][c2] != v_prev {
                        r2 = -1;
                    } else {
                        while r2 + 1 <= r2_prev && masu[(r2 + 1) as usize][c2] == v_prev {
                            r2 += 1;
                        }
                    }
                }
                dp2[c1][c2][r1][0] = r2;
                //println!("dp2[{}][{}][{}][{}] = {}", c1, c2, r1, 0, r2);
            }
        }
    }

    if dp2[0][w - 1][0][0] == (h - 1) as i32 {
        return 0;
    }

    // DP
    for f in 1..max_f {
        for r1 in 0..h {
            for r2 in r1..h {
                for c1 in 0..w {
                    // Tate
                    let c2_tate = tate_wari2(w, &dp1, r1, r2, c1, f);
                    let c2 = if r2 == r1 {
                        c2_tate
                    } else {
                        // Yoko
                        let mut c2_yoko = c2_tate;
                        if c2_yoko < 0 {
                            let r2_c1 = yoko_wari2(h, &dp2, c1, c1, r1, f);
                            if r2_c1 >= r2 as i32 {
                                c2_yoko = c1 as i32;
                            }
                        }

                        if c2_yoko >= 0 {
                            while ((c2_yoko + 1) as usize) < w {
                                let r2_next =
                                    yoko_wari2(h, &dp2, c1, (c2_yoko + 1) as usize, r1, f);
                                if r2_next >= r2 as i32 {
                                    c2_yoko += 1;
                                    continue;
                                }
                                break;
                            }
                        }

                        cmp::max(c2_tate, c2_yoko)
                    };
                    dp1[r1][r2][c1][f % 2] = c2;

                    //println!(
                    //    "dp1[{}][{}][{}][{}] = {}",
                    //    r1, r2, c1, f, dp1[r1][r2][c1][f%2]
                    //);
                }
            }
        }

        if dp1[0][h - 1][0][f % 2] == (w - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }

        for c1 in 0..w {
            for c2 in c1..w {
                for r1 in 0..h {
                    // Yoko
                    let r2_yoko = yoko_wari2(h, &dp2, c1, c2, r1, f);
                    let r2 = if c2 == c1 {
                        r2_yoko
                    } else {
                        // Tate
                        let mut r2_tate = r2_yoko;
                        if r2_tate < 0 {
                            let c2_r1 = tate_wari2(w, &dp1, r1, r1, c1, f);
                            if c2_r1 >= c2 as i32 {
                                r2_tate = r1 as i32;
                            }
                        }
                        if r2_tate >= 0 {
                            while ((r2_tate + 1) as usize) < h {
                                let c2_next =
                                    tate_wari2(w, &dp1, r1, (r2_tate + 1) as usize, c1, f);
                                if c2_next >= c2 as i32 {
                                    r2_tate += 1;
                                    continue;
                                }
                                break;
                            }
                        }

                        cmp::max(r2_yoko, r2_tate)
                    };
                    dp2[c1][c2][r1][f % 2] = r2;

                    //println!(
                    //    "dp2[{}][{}][{}][{}] = {}",
                    //    c1, c2, r1, f, dp2[c1][c2][r1][f%2]
                    //);
                }
            }
        }

        if dp2[0][w - 1][0][f % 2] == (h - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }
    }

    //for f in 0..max_f {
    //    if dp1[0][h - 1][0][f] == (w - 1) as i32 {
    //        return f as i32;
    //    }
    //}
    // if h = 1, w = 1
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

fn tate_wari2(
    w: usize,
    dp1: &[Vec<Vec<Vec<i32>>>],
    r1: usize,
    r2: usize,
    c1: usize,
    f: usize,
) -> i32 {
    let c_x = dp1[r1][r2][c1][(f - 1) % 2];
    if c_x < 0 {
        -1
    } else if c_x >= (w - 1) as i32 {
        (w - 1) as i32
    } else {
        let c_xx = dp1[r1][r2][(c_x + 1) as usize][(f - 1) % 2];
        if c_xx < 0 {
            c_x
        } else {
            c_xx
        }
    }
}

fn yoko_wari2(
    h: usize,
    dp2: &[Vec<Vec<Vec<i32>>>],
    c1: usize,
    c2: usize,
    r1: usize,
    f: usize,
) -> i32 {
    let r_x = dp2[c1][c2][r1][(f - 1) % 2];
    if r_x < 0 {
        -1
    } else if r_x >= (h - 1) as i32 {
        (h - 1) as i32
    } else {
        let r_xx = dp2[c1][c2][(r_x + 1) as usize][(f - 1) % 2];
        if r_xx < 0 {
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






/**
 * dp1[r1][r2][c1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のc2
 * dp2[c1][c2][r1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のr2
 * メモリ節約のため、fの分は現在と一個前の2個分だけ領域を確保する。
 */
fn calc2(h: usize, w: usize, masu: Vec<Vec<bool>>) -> i32 {
    let max_f = get_max_f(h, w) + 1; // exclusive
    let mut dp1 = get_4d_vec(h, h, w, 2);
    let mut dp2 = get_4d_vec(w, w, h, 2);

    // Initialization (f = 0)
    for r1 in 0..h {
        for r2 in r1..h {
            for c1 in 0..w {
                let mut c2 = c1 as i32;
                if r2 == r1 {
                    if c1 < w - 1 {
                        let v = masu[r1][c1];
                        if c1 > 0 && v == masu[r1][c1 - 1] {
                            c2 = dp1[r1][r2][c1 - 1][0];
                        } else {
                            while c2 < (w - 1) as i32 && masu[r1][(c2 + 1) as usize] == v {
                                c2 += 1;
                            }
                        }
                    }
                } else {
                    let c2_prev = dp1[r1][r2 - 1][c1][0];
                    let v_prev = masu[r2 - 1][c1];
                    if c2_prev < 0 || masu[r2][c1] != v_prev {
                        c2 = -1;
                    } else {
                        let c2_prev = dp1[r1][r2 - 1][c1][0];
                        while c2 + 1 <= c2_prev && masu[r2][(c2 + 1) as usize] == v_prev {
                            c2 += 1;
                        }
                    }
                }
                dp1[r1][r2][c1][0] = c2;
                //println!("dp1[{}][{}][{}][{}] = {}", r1, r2, c1, 0, c2);
            }
        }
    }

    if dp1[0][h - 1][0][0] == (w - 1) as i32 {
        return 0;
    }

    for c1 in 0..w {
        for c2 in c1..w {
            for r1 in 0..h {
                let mut r2 = r1 as i32;
                if c2 == c1 {
                    if r1 < h - 1 {
                        let v = masu[r1][c1];
                        if r1 > 0 && v == masu[r1 - 1][c1] {
                            r2 = dp2[c1][c2][r1 - 1][0];
                        } else {
                            while r2 < (h - 1) as i32 && masu[(r2 + 1) as usize][c1] == v {
                                r2 += 1;
                            }
                        }
                    }
                } else {
                    let r2_prev = dp2[c1][c2 - 1][r1][0];
                    let v_prev = masu[r1][c2 - 1];
                    if r2_prev < 0 || masu[r1][c2] != v_prev {
                        r2 = -1;
                    } else {
                        while r2 + 1 <= r2_prev && masu[(r2 + 1) as usize][c2] == v_prev {
                            r2 += 1;
                        }
                    }
                }
                dp2[c1][c2][r1][0] = r2;
                //println!("dp2[{}][{}][{}][{}] = {}", c1, c2, r1, 0, r2);
            }
        }
    }

    if dp2[0][w - 1][0][0] == (h - 1) as i32 {
        return 0;
    }

    // DP
    for f in 1..max_f {
        for r1 in 0..h {
            for r2 in r1..h {
                for c1 in 0..w {
                    // Tate
                    let c2_tate = tate_wari2(w, &dp1, r1, r2, c1, f);

                    // Yoko
                    let mut c2_yoko = c2_tate;
                    if c2_yoko < 0 {
                        let r2_c1 = yoko_wari2(h, &dp2, c1, c1, r1, f);
                        if r2_c1 >= r2 as i32 {
                            c2_yoko = c1 as i32;
                        }
                    }

                    if c2_yoko >= 0 {
                        while ((c2_yoko + 1) as usize) < w {
                            let r2_next = yoko_wari2(h, &dp2, c1, (c2_yoko + 1) as usize, r1, f);
                            if r2_next >= r2 as i32 {
                                c2_yoko += 1;
                                continue;
                            }
                            break;
                        }
                    }

                    dp1[r1][r2][c1][f % 2] = cmp::max(c2_tate, c2_yoko);

                    //println!(
                    //    "dp1[{}][{}][{}][{}] = {}",
                    //    r1, r2, c1, f, dp1[r1][r2][c1][f%2]
                    //);
                }
            }
        }

        if dp1[0][h - 1][0][f % 2] == (w - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }

        for c1 in 0..w {
            for c2 in c1..w {
                for r1 in 0..h {
                    // Yoko
                    let r2_yoko = yoko_wari2(h, &dp2, c1, c2, r1, f);

                    // Tate
                    let mut r2_tate = r2_yoko;
                    if r2_tate < 0 {
                        let c2_r1 = tate_wari2(w, &dp1, r1, r1, c1, f);
                        if c2_r1 >= c2 as i32 {
                            r2_tate = r1 as i32;
                        }
                    }
                    if r2_tate >= 0 {
                        while ((r2_tate + 1) as usize) < h {
                            let c2_next = tate_wari2(w, &dp1, r1, (r2_tate + 1) as usize, c1, f);
                            if c2_next >= c2 as i32 {
                                r2_tate += 1;
                                continue;
                            }
                            break;
                        }
                    }

                    dp2[c1][c2][r1][f % 2] = cmp::max(r2_yoko, r2_tate);
                    //println!(
                    //    "dp2[{}][{}][{}][{}] = {}",
                    //    c1, c2, r1, f, dp2[c1][c2][r1][f%2]
                    //);
                }
            }
        }

        if dp2[0][w - 1][0][f % 2] == (h - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }
    }

    //for f in 0..max_f {
    //    if dp1[0][h - 1][0][f] == (w - 1) as i32 {
    //        return f as i32;
    //    }
    //}
    // if h = 1, w = 1
    return 0;
}

/**
 * dp1[r1][r2][c1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のc2
 * dp2[c1][c2][r1][f]: 左上が(r1, c1),  右下が(r2, c2)の長方形で複雑度がf以下になる最大のr2
 */
fn calc(h: usize, w: usize, masu: Vec<Vec<bool>>) -> i32 {
    let max_f = get_max_f(h, w) + 1; // exclusive
    let mut dp1 = get_4d_vec(h, h, w, max_f);
    let mut dp2 = get_4d_vec(w, w, h, max_f);

    // f = 0
    for r1 in 0..h {
        for r2 in r1..h {
            for c1 in 0..w {
                let mut c2 = c1 as i32;
                if r2 == r1 {
                    if c1 < w - 1 {
                        let v = masu[r1][c1];
                        if c1 > 0 && v == masu[r1][c1 - 1] {
                            c2 = dp1[r1][r2][c1 - 1][0];
                        } else {
                            while c2 < (w - 1) as i32 && masu[r1][(c2 + 1) as usize] == v {
                                c2 += 1;
                            }
                        }
                    }
                } else {
                    let c2_prev = dp1[r1][r2 - 1][c1][0];
                    let v_prev = masu[r2 - 1][c1];
                    if c2_prev < 0 || masu[r2][c1] != v_prev {
                        c2 = -1;
                    } else {
                        let c2_prev = dp1[r1][r2 - 1][c1][0];
                        while c2 + 1 <= c2_prev && masu[r2][(c2 + 1) as usize] == v_prev {
                            c2 += 1;
                        }
                    }
                }
                dp1[r1][r2][c1][0] = c2;
                //println!("dp1[{}][{}][{}][{}] = {}", r1, r2, c1, 0, c2);
            }
        }
    }

    for c1 in 0..w {
        for c2 in c1..w {
            for r1 in 0..h {
                let mut r2 = r1 as i32;
                if c2 == c1 {
                    if r1 < h - 1 {
                        let v = masu[r1][c1];
                        if r1 > 0 && v == masu[r1 - 1][c1] {
                            r2 = dp2[c1][c2][r1 - 1][0];
                        } else {
                            while r2 < (h - 1) as i32 && masu[(r2 + 1) as usize][c1] == v {
                                r2 += 1;
                            }
                        }
                    }
                } else {
                    let r2_prev = dp2[c1][c2 - 1][r1][0];
                    let v_prev = masu[r1][c2 - 1];
                    if r2_prev < 0 || masu[r1][c2] != v_prev {
                        r2 = -1;
                    } else {
                        while r2 + 1 <= r2_prev && masu[(r2 + 1) as usize][c2] == v_prev {
                            r2 += 1;
                        }
                    }
                }
                dp2[c1][c2][r1][0] = r2;
                //println!("dp2[{}][{}][{}][{}] = {}", c1, c2, r1, 0, r2);
            }
        }
    }

    // DP
    for f in 1..max_f {
        for r1 in 0..h {
            for r2 in r1..h {
                for c1 in 0..w {
                    // Tate
                    let c2_tate = tate_wari(w, &dp1, r1, r2, c1, f);

                    // Yoko
                    let mut c2_yoko = c2_tate;
                    if c2_yoko < 0 {
                        let r2_c1 = yoko_wari(h, &dp2, c1, c1, r1, f);
                        if r2_c1 >= r2 as i32 {
                            c2_yoko = c1 as i32;
                        }
                    }

                    if c2_yoko >= 0 {
                        while ((c2_yoko + 1) as usize) < w {
                            let r2_next = yoko_wari(h, &dp2, c1, (c2_yoko + 1) as usize, r1, f);
                            if r2_next >= r2 as i32 {
                                c2_yoko += 1;
                                continue;
                            }
                            break;
                        }
                    }

                    dp1[r1][r2][c1][f] = cmp::max(c2_tate, c2_yoko);
                    //println!(
                    //  "dp1[{}][{}][{}][{}] = {}",
                    //r1, r2, c1, f, dp1[r1][r2][c1][f]
                    //);
                }
            }
        }

        if dp1[0][h - 1][0][f] == (w - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }

        for c1 in 0..w {
            for c2 in c1..w {
                for r1 in 0..h {
                    // Yoko
                    let r2_yoko = yoko_wari(h, &dp2, c1, c2, r1, f);

                    // Tate
                    let mut r2_tate = r2_yoko;
                    if r2_tate < 0 {
                        let c2_r1 = tate_wari(w, &dp1, r1, r1, c1, f);
                        if c2_r1 >= c2 as i32 {
                            r2_tate = r1 as i32;
                        }
                    }
                    if r2_tate >= 0 {
                        while ((r2_tate + 1) as usize) < h {
                            let c2_next = tate_wari(w, &dp1, r1, (r2_tate + 1) as usize, c1, f);
                            if c2_next >= c2 as i32 {
                                r2_tate += 1;
                                continue;
                            }
                            break;
                        }
                    }

                    dp2[c1][c2][r1][f] = cmp::max(r2_yoko, r2_tate);
                    //println!(
                    //  "dp2[{}][{}][{}][{}] = {}",
                    //c1, c2, r1, f, dp2[c1][c2][r1][f]
                    //);
                }
            }
        }

        if dp2[0][w - 1][0][f] == (h - 1) as i32 {
            //println!("Bingo!");
            return f as i32;
        }
    }

    //for f in 0..max_f {
    //    if dp1[0][h - 1][0][f] == (w - 1) as i32 {
    //        return f as i32;
    //    }
    //}
    // if h = 1, w = 1
    return 0;
}

fn tate_wari(
    w: usize,
    dp1: &[Vec<Vec<Vec<i32>>>],
    r1: usize,
    r2: usize,
    c1: usize,
    f: usize,
) -> i32 {
    let c_x = dp1[r1][r2][c1][f - 1];
    if c_x < 0 {
        -1
    } else if c_x >= (w - 1) as i32 {
        (w - 1) as i32
    } else {
        dp1[r1][r2][(c_x + 1) as usize][f - 1]
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
    let r_x = dp2[c1][c2][r1][f - 1];
    if r_x < 0 {
        -1
    } else if r_x >= (h - 1) as i32 {
        (h - 1) as i32
    } else {
        dp2[c1][c2][(r_x + 1) as usize][f - 1]
    }
}
