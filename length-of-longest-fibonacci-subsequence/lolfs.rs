pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            let aa = helper(arr[i], arr[j], arr.get(j + 1..).unwrap());
            if aa != 0 {
                if aa + 2 > result {
                    result = aa + 2
                }
            }
        }
    }
    result
}

fn helper(i0: i32, i1: i32, arrs: &[i32]) -> i32 {
    let mut result = 0;
    for ind in 0..arrs.len() {
        if arrs[ind] == i0 + i1 {
            let aa = 1 + helper(i1, arrs[ind], arrs.get(ind..).unwrap());
            if aa > result {
                result = aa
            }
        }
    }
    result
}

fn main() {
    //dbg!(helper(1, 2, &[3, 4, 5, 6, 7, 8]));
    assert_eq!(0, len_longest_fib_subseq(vec![1, 3, 5]));
    assert_eq!(5, len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq!(3, len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]));
}
