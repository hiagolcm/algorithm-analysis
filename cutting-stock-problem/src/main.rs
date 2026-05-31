const T: [u32;11] = [0,1,5,8,9,10,17, 17,20,24,30];

fn csp_divide_and_conquer(n: usize) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut max: u32 = 0;

    for i in 1..n {
        let total = csp_divide_and_conquer(i) + csp_divide_and_conquer(n-i);
        
        if max < total {
            max = total;
        }
    }

    if n < 11 && T[n] > max {
        return T[n];
    }

    return max;
}

fn csp_dynamic(n: usize) -> u32 {
    let mut t: Vec<u32> = vec![0; n+1];

    for i in 0..n + 1 {
        csp_aux(i, &mut t);
    }

    dbg!(&t);

    return t[n];
}

fn csp_aux(n: usize, t: &mut Vec<u32>) {
    if n == 0 {
        return;
    }

    let mut max: u32 = 0;

    if n <= 10 {
        max = T[n];
    }

    for i in 1..n {
        let total = t[i] + t[n-i];
        if total > max {
            max = total;
        }
    }

    t[n] = max;
}

fn main() {
    println!("{}", csp_dynamic(20));
    // println!("{}", csp_divide_and_conquer(20));
}
