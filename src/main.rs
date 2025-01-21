extern crate nqueen;

use core::panic;
use std::{env, process::exit, time::Instant};

use nqueen::TimeKeeper;
use rand::Rng;

#[inline(always)]
fn show_board(sol: &Vec<usize>) {
    let n: usize = sol.len();
    for &val in sol {
        for j in 0..n {
            if val == j {
                print!("o ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

#[inline(always)]
fn show_log(sol: &Vec<usize>, diag_up: &Vec<usize>, diag_dn: &Vec<usize>) {
    println!("array: {:?}", sol);
    show_board(sol);
    println!("queens on upward diagonals: {:?}", diag_up);
    println!("queens on downward diagonals: {:?}", diag_dn);
    let n_colls = collisions(diag_up) + collisions(diag_dn);
    println!("#collisions: {n_colls}");
    println!();
}

#[inline(always)]
fn collisions(diag: &Vec<usize>) -> usize {
    let mut n_colls = 0;
    for &i in diag {
        if i > 1 {
            n_colls += i - 1;
        }
    }
    n_colls
}

#[inline(always)]
fn exchange(
    i: usize,
    j: usize,
    sol: &mut Vec<usize>,
    diag_up: &mut [usize],
    diag_dn: &mut [usize],
) {
    let n = sol.len();

    let d = i + sol[i];
    diag_up[d] -= 1;
    let d = j + sol[j];
    diag_up[d] -= 1;

    let d = (n - 1) - i + sol[i];
    diag_dn[d] -= 1;
    let d = (n - 1) - j + sol[j];
    diag_dn[d] -= 1;

    // exchange the positions 'i' and 'j'
    sol.swap(i, j);

    // diagonals that started being attacked
    let d = i + sol[i];
    diag_up[d] += 1;
    let d = j + sol[j];
    diag_up[d] += 1;

    let d = (n - 1) - i + sol[i];
    diag_dn[d] += 1;
    let d = (n - 1) - j + sol[j];
    diag_dn[d] += 1;
}

#[inline(always)]
fn construct(sol: &mut Vec<usize>, time_keeper: &TimeKeeper) -> (Vec<usize>, Vec<usize>) {
    let n = sol.len();
    let n_diag = 2 * n - 1;

    // # upward diagonals (index 0 corresponds to the diagonal on upper-left square)
    let mut diag_up: Vec<usize> = vec![0; n_diag];

    // # downward diagonals (index 0 corresponds to the diagonal on upper-right square)
    let mut diag_dn: Vec<usize> = vec![0; n_diag];

    let mut cand: Vec<usize> = (0..n).collect();
    let trials: usize = (10.0 * (n as f64).log10()) as usize;
    for i in 0..n {
        if time_keeper.is_time_over() {
            return (diag_up, diag_dn);
        }
        let mut forelse: bool = true;
        for _ in 0..trials {
            let col_id: usize = rand::thread_rng().gen_range(0..cand.len());
            let col: usize = cand[col_id];
            let n_colls: usize = diag_up[i + col] + diag_dn[(n - 1) - i + col];
            if n_colls == 0 {
                sol[i] = col;
                diag_up[i + col] += 1;
                diag_dn[(n - 1) - i + col] += 1;
                cand[col_id] = cand[cand.len() - 1];
                cand.pop();
                forelse = false;
                break;
            }
        }
        if forelse {
            let mut min_colls = 100000000;
            let mut col_id: Option<usize> = None;
            let mut col: Option<usize> = None;
            for j in 0..cand.len() {
                let n_colls = diag_up[i + cand[j]] + diag_dn[(n - 1) - i + cand[j]];
                if n_colls < min_colls {
                    min_colls = n_colls;
                    col = Some(cand[j]);
                    col_id = Some(j);
                }
            }
            match col {
                None => {
                    panic!("couldn't update value");
                }
                Some(j) => {
                    sol[i] = j;
                    diag_up[i + j] += 1;
                    diag_dn[(n - 1) - i + j] += 1;
                }
            }
            match col_id {
                None => {
                    panic!("couldn't update value");
                }
                Some(j) => {
                    cand.remove(j);
                }
            }
        }
    }
    (diag_up, diag_dn)
}

#[inline(always)]
fn fast_tabu_search(sol: &mut Vec<usize>, diag_up: &mut [usize], diag_dn: &mut [usize]) {
    let n: usize = sol.len();
    let mut tabu: Vec<Option<usize>> = vec![None; n];
    let mut tabulen = std::cmp::min(10, n);
    let mut n_iter: usize = 0;
    loop {
        n_iter += 1;
        let mut forelse: bool = true;
        let mut i_star: usize = 0;
        let mut colls_star: usize = 0;
        for i in (0..n).rev() {
            let n_colls = diag_up[i + sol[i]] + diag_dn[(n - 1) - i + sol[i]];
            if n_colls - 2 > 0 {
                i_star = i;
                colls_star = n_colls;
                forelse = false;
                break;
            }
        }
        if forelse {
            // no collusion, we finish the search
            break;
        }

        let mut delta = 0;
        let mut j_star: Option<usize> = None;
        for j in 0..n {
            match tabu[j] {
                None => {}
                Some(t) => {
                    if t >= n_iter || j == i_star {
                        continue;
                    }
                }
            }

            let temp = (diag_up[j + sol[j]] + diag_dn[(n - 1) - j + sol[j]] + colls_star)
                - (diag_up[i_star + sol[j]]
                    + diag_dn[(n - 1) - i_star + sol[j]]
                    + diag_up[j + sol[i_star]]
                    + diag_dn[(n - 1) - j + sol[i_star]]);
            if temp > delta {
                delta = temp;
                j_star = Some(j);
            }
        }

        match j_star {
            None => {
                // clear tabu list
                tabulen = tabulen / 2 + 1;
                tabu = vec![None; n];
            }
            Some(j_star) => {
                println!("iter={n_iter}: swap {i_star}&{j_star}, delta={delta}");
                let val = n_iter + rand::thread_rng().gen_range(1..tabulen);
                tabu[i_star] = Some(val);
                tabu[j_star] = Some(val);
                exchange(i_star, j_star, sol, diag_up, diag_dn);
            }
        }
    }
}

fn main() {
    let start: Instant = Instant::now();

    let log: bool = false;

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("$1: #queens");
        println!("$2: time threshold [sec]");
        exit(1);
    }

    let time_threshold_seconds: u64 = args[2].parse().unwrap();
    let time_keeper: TimeKeeper = TimeKeeper {
        start_time: Instant::now(),
        time_threshold_seconds,
    };

    let n: usize = args[1].parse().unwrap();

    let mut sol: Vec<_> = (0..n).collect();

    let (mut up, mut dn) = construct(&mut sol, &time_keeper);

    println!("--------- Initial solution (random greedy) ---------");
    let mut n_colls = collisions(&up) + collisions(&dn);
    println!("#collision: {n_colls}");
    if log {
        show_log(&sol, &up, &dn);
    }

    println!("--------- starting fast tabu search ---------");
    fast_tabu_search(&mut sol, &mut up, &mut dn);
    println!("--------- finish fast tabu search ---------");
    n_colls = collisions(&up) + collisions(&dn);
    println!("#collision: {n_colls}",);
    if log {
        show_log(&sol, &up, &dn);
    }

    println!("Time elapsed: {:?}", start.elapsed());
}
