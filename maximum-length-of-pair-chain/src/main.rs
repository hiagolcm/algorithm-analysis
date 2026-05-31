use std::collections::HashMap;


struct Solution {}

fn sort_vect(pairs: &mut Vec<Vec<i32>>) {
    pairs.sort_by(|a, b| 
        a[1].cmp(&b[1])
    );
}

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        if pairs.len() == 0 {
            return 0;
        }

        let mut sorted_pairs = pairs.clone();
        sort_vect(&mut sorted_pairs);

        let mut total = 1;
        let mut k = sorted_pairs[0][1];


        for p in sorted_pairs[1..].iter() {
            if p[0] <= k {
                continue
            }

            k = p[1];
            total += 1;
        }

        return total;
    }

    pub fn find_longest_chain_slow(pairs: Vec<Vec<i32>>) -> i32 {
        let mut sorted_pairs = pairs.clone();
        sort_vect(&mut sorted_pairs);

        let i = sorted_pairs[0][0];
        let j = sorted_pairs[sorted_pairs.len() - 1][1];

        let mut t: HashMap<(i32, i32), i32> = HashMap::new();

        Solution::find_longest_chain_slow_aux(&sorted_pairs, i, j, &mut t)
    }

    pub fn find_longest_chain_slow_aux(pairs: &Vec<Vec<i32>>, i: i32, j: i32, mut t: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(val) = t.get(&(i,j)) {
            return *val; 
        }

        let mut solution = 0;

        for p in pairs {
            if p[0] > j || p[0] < i || p[1] > j || p[1] < i {
                continue;
            }

            let longest = Solution::find_longest_chain_slow_aux(pairs, i, p[0] -1, &mut t) + Solution::find_longest_chain_slow_aux(pairs, p[1] + 1, j, &mut t) + 1;
            t.insert((i,j), longest);

            if longest > solution {
                solution = longest;
            }
        }

        solution
    }
}