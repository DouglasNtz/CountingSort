
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

pub fn radix_sort(v: &mut Vec<usize>) {

    let mut c = vec![0; 10];

    let max = v.iter().max().unwrap().ilog10() as usize;

    for casa_decimal in 0..=max {

        let mut w = vec![];

        let rv = &(*v);

        for &i in rv {

            w.push(((i/(10_usize.pow(casa_decimal as u32))) % 10, i));
        }

        counting_sort_proof_stability(&mut w, 9);

        v.clear();

        for (i , j) in w {
            v.push(j);
        }
    }

}

pub fn radix_sort_limit_digits(v: &mut Vec<usize>, max_digits: usize) {

    let mut c = vec![0; 10];

    for casa_decimal in 0..max_digits {

        let mut w = vec![];

        let rv = &(*v);

        for &i in rv {

            w.push(((i/(10_usize.pow(casa_decimal as u32))) % 10, i));
        }

        counting_sort_proof_stability(&mut w, 9);

        v.clear();

        for (i , j) in w {
            v.push(j);
        }
    }

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