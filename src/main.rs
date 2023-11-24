extern crate nqueen;

use rand::Rng;

fn show_board(sol: &Vec<usize>) {
    let n: usize = sol.len();
    // println!("{:?}", sol);
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

fn diagonals(sol: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let n = sol.len();
    let n_diag = 2 * n - 1;

    let mut diag_up: Vec<_> = (0..n_diag).collect();
    let mut diag_dn: Vec<_> = (0..n_diag).collect();

    for i in 0..n {
        let d = i + sol[i];
        diag_up[d] += 1;

        let d = (n - 1) + sol[i] - i;
        diag_dn[d] += 1;
    }
    (diag_up, diag_dn)
}

fn collisions(diag: &Vec<usize>) -> usize {
    let mut n_colls = 0;
    for &i in diag {
        if i > 1 {
            n_colls += i - 1;
        }
    }
    n_colls
}

fn exchange(
    i: usize,
    j: usize,
    sol: &mut Vec<usize>,
    diag_up: &mut Vec<usize>,
    diag_dn: &mut Vec<usize>,
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

fn construct(sol: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let n = sol.len();
    let n_diag = 2 * n - 1;

    // # upward diagonals (index 0 corresponds to the diagonal on upper-left square)
    let mut diag_up: Vec<usize> = (0..n_diag).collect();

    // # downward diagonals (index 0 corresponds to the diagonal on upper-right square)
    let mut diag_dn: Vec<usize> = (0..n_diag).collect();

    let mut cand: Vec<usize> = (0..n).collect();
    let trials = (10.0 * (n as f64).log10()) as usize;
    for i in 0..n {
        let mut forelse = true;
        for t in 0..trials {
            let col_id = rand::thread_rng().gen_range(0..cand.len());
            let col = cand[col_id];
            let n_colls = diag_up[i + col] + diag_dn[(n - 1) - i + col];
            if n_colls == 0 {
                sol[i] = col;
                diag_up[i + col] += 1;
                diag_dn[(n - 1) - i + col] += 1;
                cand.remove(col_id);
                forelse = false;
                break;
            }
        }
        if forelse {
            let mut min_colls = 100000000;
            let mut col_id = -1;
            let mut col = -1;
            for j in 0..cand.len() {
                let n_colls = diag_up[i + cand[j]] + diag_dn[(n - 1) - i + cand[j]];
                if n_colls < min_colls {
                    min_colls = n_colls;
                    col = cand[j];
                    col_id = j
                }
            }
            sol[i] = col;
            diag_up[i + col] += 1;
            diag_dn[(n - 1) - i + col] += 1;
            cand.remove(col_id);
        }
    }
    (diag_up, diag_dn)
}

fn fast_tabu_search(sol: &Vec<usize>, diag_up: &Vec<usize>, diag_dn: &Vec<usize>){
    let n = sol.len();
    let mut tabu: Vec<usize> = vec![-1, n];
    let max_iter = 100000;
    let mut tabulen = std::cmp::min(10, n);
    for n_iter in 0..max_iter{
        for i in range(n - 1, -1, -1):
            colls = diag_up[i + sol[i]] + diag_dn[(n - 1) - i + sol[i]]
            if colls - 2 > 0:
                istar = i
                break
        else:  # no collusion, we finish the search
            break

        # print "swap candidate is",istar
        delta = -999999
        jstar = -1
        for j in range(n):
            if tabu[j] >= n_iter or j == istar:
                continue
            temp = (diag_up[j + sol[j]] + diag_dn[(n - 1) - j + sol[j]] + colls) - (
                diag_up[istar + sol[j]]
                + diag_dn[(n - 1) - istar + sol[j]]
                + diag_up[j + sol[istar]]
                + diag_dn[(n - 1) - j + sol[istar]]
            )
            if temp > delta:
                delta = temp
                jstar = j

        print("iter=", n_iter, "swap", istar, jstar, "delta=", delta)
        if jstar == -1:  # clear tabu list
            tabulen = int(tabulen / 2) + 1
            tabu = [-1] * n
        else:
            tabu[istar] = tabu[jstar] = n_iter + random.randint(1, tabulen)
            exchange(istar, jstar, sol, diag_up, diag_dn)

        if LOG:
            display(sol)
            up, dn = diagonals(sol)
            print("queens on upward diagonals:", up)
            print("queens on downward diagonals:", dn)
            ncolls = collisions(up) + collisions(dn)
    }

    ()
}

fn main() {
    let LOG = false;

    let n = 10;

    let sol: Vec<_> = (0..n).collect();

    println!("{:?}", sol);

    show_board(&sol);
}
