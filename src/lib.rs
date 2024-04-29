#[cfg(test)]
mod tests;

pub fn counting_sort(v: &mut Vec<usize>, k: usize) {

    let mut c = vec![0; k + 1];

    let r = &(*v);

    for &i in r {
        c[i] += 1;
    }

    for j in 1..=k {
        c[j] = c[j] + c[j - 1];
    }

    let mut w = vec![0; v.len()];

    loop {

        match v.pop() {

            Some(i) => {

                w[c[i] - 1] = i;

                c[i] -= 1;
            }

            None => break
        }
    }

    v.append(&mut w);

}

pub fn counting_sort_proof_stability(v: &mut Vec<(usize, usize)>, k: usize) {

    let mut c = vec![0; k + 1];

    let r = &(*v);

    for (i, j) in r {
        c[*i] += 1;
    }

    for j in 1..=k {
        c[j] = c[j] + c[j - 1];
    }

    let mut w = vec![(0,0); v.len()];

    loop {

        match v.pop() {

            Some((i, j)) => {

                w[c[i] - 1] = (i, j);

                c[i] -= 1;
            }

            None => break
        }
    }

    v.append(&mut w);

}

pub fn selection_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    for k in 0..(v.len() - 1) {

        let mut min = v[k];

        let mut pos = k;

        for i in (k + 1)..v.len() {

            if v[i] < min {

                min = v[i];

                pos = i

            }
        }

        v.swap(pos, k)
    }
}

pub fn selection_sort_proof_stability<T: PartialOrd + Copy>(v: &mut Vec<(T, usize)>) {

    for k in 0..(v.len() - 1) {

        let mut min = v[k].0;

        let mut pos = k;

        for i in (k + 1)..v.len() {

            if v[i].0 < min {

                min = v[i].0;

                pos = i

            }
        }

        v.swap(pos, k)
    }
}