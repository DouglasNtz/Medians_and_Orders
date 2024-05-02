#[cfg(test)]
mod tests;

use std::ops::Range;

// Em caso de empate, pegamos o primeiro min. Ou seja, o primeiro elemento dado um sorting estável
pub fn return_min<T: PartialOrd + Copy>(v: &[T]) -> T {

    let mut vmin = v[0];

    for x in &v[1..] {
        if *x < vmin {
            vmin = *x;
        }
    }
    vmin
}

fn return_min_proof_stability<T: PartialOrd + Copy>(v: &[(T, usize)]) -> (T, usize) {

    let mut vmin = v[0];

    for &(x, i) in &v[1..] {
        if x < vmin.0 {
            vmin = (x, i);
        }
    }
    vmin
}

// Em caso de empate, pegamos o último max. Ou seja, o último elemento dado um sorting estável
pub fn return_max<T: PartialOrd + Copy>(v: &[T]) -> T {

    let mut vmax = v[0];

    for x in &v[1..] {
        if *x >= vmax {
            vmax = *x;
        }
    }
    vmax
}

fn return_max_proof_stability<T: PartialOrd + Copy>(v: &[(T, usize)]) -> (T, usize) {

    let mut vmax = v[0];

    for &(x, i) in &v[1..] {
        if x >= vmax.0 {
            vmax = (x, i);
        }
    }
    vmax
}

pub fn return_min_max<T: PartialOrd + Copy>(v: &[T]) -> (T, T) {

    let mut vmin = v[0];
    let mut vmax = v[0];

    for x in &v[1..] {
        if *x < vmin {
            vmin = *x;
        } else if *x >= vmax {
            vmax = *x;
        }
    }
    (vmin, vmax)
}

fn return_min_max_proof_stability<T: PartialOrd + Copy>(v: &[(T, usize)]) -> ((T,usize), (T, usize)) {

    let mut vmin = v[0];
    let mut vmax = v[0];

    for &(x, i) in &v[1..] {
        if x < vmin.0 {
            vmin = (x, i);
        } else if x >= vmax.0 {
            vmax = (x, i);
        }
    }
    (vmin, vmax)
}

pub fn return_min_max_fast<T: PartialOrd + Copy>(v: &[T]) -> (T, T) {

    let mut vmin;
    let mut vmax;

    if v.len() % 2 == 0 {

        if v[0] <= v[1] {
            vmin = v[0];
            vmax = v[1];
        } else {
            vmin = v[1];
            vmax = v[0];
        }

        for i in 1..v.len()/2 {

            if v[2*i] <= v[2*i + 1] {

                if v[2*i] < vmin {
                    vmin = v[2*i];
                }
                if v[2*i + 1] >= vmax {
                    vmax = v[2*i + 1];
                }

            } else {

                if v[2*i + 1] < vmin {
                    vmin = v[2*i + 1];
                }
                if v[2*i] >= vmax {
                    vmax = v[2*i];
                }
            }
        }

    } else {

        vmin = v[0];
        vmax = v[0];

        for i in 1..=v.len()/2 {

            if v[2*i - 1] <= v[2*i] {

                if v[2*i - 1] < vmin {
                    vmin = v[2*i - 1];
                }
                if v[2*i] >= vmax {
                    vmax = v[2*i];
                }

            } else {

                if v[2*i] < vmin {
                    vmin = v[2*i];
                }
                if v[2*i - 1] >= vmax {
                    vmax = v[2*i - 1];
                }

            }
        }
    }

    (vmin, vmax)
}

fn return_min_max_fast_proof_stability<T: PartialOrd + Copy>(v: &[(T, usize)]) -> ((T, usize), (T, usize)) {

    let mut vmin;
    let mut vmax;

    if v.len() % 2 == 0 {

        if v[0].0 <= v[1].0 {
            vmin = v[0];
            vmax = v[1];
        } else {
            vmin = v[1];
            vmax = v[0];
        }

        for i in 1..v.len()/2 {

            if v[2*i].0 <= v[2*i + 1].0 {

                if v[2*i].0 < vmin.0 {
                    vmin = v[2*i];
                }
                if v[2*i + 1].0 >= vmax.0 {
                    vmax = v[2*i + 1];
                }

            } else {

                if v[2*i + 1].0 < vmin.0 {
                    vmin = v[2*i + 1];
                }
                if v[2*i].0 >= vmax.0 {
                    vmax = v[2*i];
                }
            }
        }

    } else {

        vmin = v[0];
        vmax = v[0];

        for i in 1..=v.len()/2 {

            if v[2*i - 1].0 <= v[2*i].0 {

                if v[2*i - 1].0 < vmin.0 {
                    vmin = v[2*i - 1];
                }
                if v[2*i].0 >= vmax.0 {
                    vmax = v[2*i];
                }

            } else {

                if v[2*i].0 < vmin.0 {
                    vmin = v[2*i];
                }
                if v[2*i - 1].0 >= vmax.0 {
                    vmax = v[2*i - 1];
                }

            }
        }
    }

    (vmin, vmax)
}

pub use rand::random;
pub fn randomized_select<T: PartialOrd + Copy>(v: &[T], rank: usize) -> T {  // retornna o elemento de índice rank - 1 do vetor ordenado

    let vlen = v.len();

    if rank > vlen {
        panic!("A sequência possui {vlen} elementos e portanto não pode possuir um {rank}-ésimo elemento");
    }

    let mut i_rank = rank - 1; // índice que de fato queremos (da versão parcialmente ordenada)

    // seleciona um elementp dp índice i_left até i_right (último elemento)

    let mut i_rand = random::<usize>() % vlen;

    let mut vleft = Vec::<T>::with_capacity(vlen);

    let mut vright = Vec::<T>::with_capacity(vlen);

    // Do jeito que faremos, o algoritmo será estável. Foi um caso pensado para ser estável.

    for k in 0..i_rand {

        if v[k] <= v[i_rand] {
            vleft.push(v[k]);
        } else {
            vright.push(v[k]);
        }
    }

    for k in (i_rand + 1)..vlen {

        if v[k] < v[i_rand] {
            vleft.push(v[k]);
        } else {
            vright.push(v[k]);
        }
    }

    if vleft.len() == i_rank {

        return v[i_rand]

    }

    let mut w = Vec::with_capacity(vlen);

    if vleft.len() < i_rank {

        i_rank = i_rank - vleft.len() - 1;

        vleft.clear();

        w.append(&mut vright);

    } else {

        vright.clear();

        w.append(&mut vleft);

    }

    randomized_select(&w, i_rank + 1)

}

fn randomized_select_proof_stability<T: PartialOrd + Copy>(v: &[(T, usize)], rank: usize) -> (T, usize) {
// retornna o elemento de índice rank - 1 do vetor ordenado

    let vlen = v.len();

    if rank > vlen {
        panic!("A sequência possui {vlen} elementos e portanto não pode possuir um {rank}-ésimo elemento");
    }

    let mut i_rank = rank - 1; // índice que de fato queremos (da versão parcialmente ordenada)

    // seleciona um elementp dp índice i_left até i_right (último elemento)

    let mut i_rand = random::<usize>() % vlen;

    let mut vleft = Vec::<(T, usize)>::with_capacity(vlen);

    let mut vright = Vec::<(T, usize)>::with_capacity(vlen);

    // Do jeito que faremos, o algoritmo será estável. Foi um caso pensado para ser estável.

    for k in 0..i_rand {

        if v[k].0 <= v[i_rand].0 {
            vleft.push(v[k]);
        } else {
            vright.push(v[k]);
        }
    }

    for k in (i_rand + 1)..vlen {

        if v[k].0 < v[i_rand].0 {
            vleft.push(v[k]);
        } else {
            vright.push(v[k]);
        }
    }

    if vleft.len() == i_rank {

        return v[i_rand]

    }

    let mut w = Vec::<(T, usize)>::with_capacity(vlen);

    if vleft.len() < i_rank {

        i_rank = i_rank - vleft.len() - 1;

        vleft.clear();

        w.append(&mut vright);

    } else {

        vright.clear();

        w.append(&mut vleft);

    }

    randomized_select_proof_stability(&w, i_rank + 1)

}