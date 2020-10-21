use std::collections::{HashMap, HashSet};

pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let mut record: HashMap<usize, usize> = HashMap::new(); // ind -> count
    for v in &requests {
        for ind in v[0]..=v[1] {
            let a = record.entry(ind as usize).or_insert(0);
            *a += 1
        }
    }

    let mut count_set = HashSet::new();
    let mut count_ind: HashMap<usize, Vec<usize>> = HashMap::new(); // count -> vec<ind>
    for (k, v) in record.iter() {
        let a = count_ind.entry(*v).or_insert(vec![]);
        a.push(*k);
        count_set.insert(v);
    }

    let mut count_set = count_set.into_iter().map(|a| *a).collect::<Vec<usize>>();
    count_set.sort();
    count_set.reverse();
    //dbg!(&count_ind);
    //dbg!(&count_set);

    // sort nums
    let mut sort_nums = nums.clone();
    sort_nums.sort();
    sort_nums.reverse(); // larger to smaller
                         //dbg!(&sort_nums);
    let mut sort_nums_iter = sort_nums.iter();

    let mut whole_vec = vec![0; 100000];
    for v in count_set {
        for ind in count_ind.get(&v).unwrap() {
            whole_vec[*ind] = *sort_nums_iter.next().unwrap()
        }
    }

    //dbg!(&whole_vec[0..10]);

    // get result
    let mut result = 0;
    for ind_range in requests {
        result = result % 1000000007
            + (ind_range[0]..=ind_range[1])
                .map(|ind| whole_vec[ind as usize] % 1000000007)
                .sum::<i32>()
                % 1000000007;
    }

    result
}

// first one is too slow, obiviously
pub fn max_sum_range_query2(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let mut record: HashMap<usize, usize> = HashMap::new(); // ind -> count
    for v in &requests {
        for ind in v[0]..=v[1] {
            let a = record.entry(ind as usize).or_insert(0);
            *a += 1
        }
    }

    let mut count_set = HashSet::new();
    let mut count_ind: HashMap<usize, usize> = HashMap::new(); // count -> number of ind has this count
    for (_, v) in record.iter() {
        let a = count_ind.entry(*v).or_insert(0);
        *a += 1;
        count_set.insert(v);
    }

    let mut count_set = count_set.into_iter().map(|a| *a).collect::<Vec<usize>>();
    count_set.sort();
    count_set.reverse();

    // sort nums
    let mut sort_nums = nums.clone();
    sort_nums.sort();
    sort_nums.reverse(); // larger to smaller
    let mut sort_nums_iter = sort_nums.iter();

    let mut result = 0;
    for count in count_set {
        for _ in 0_usize..*count_ind.get(&count).unwrap() {
            result = result % 1000000007
                + (count as i32 * sort_nums_iter.next().unwrap() % 1000000007) % 1000000007
        }
    }
    result
}

pub fn max_sum_range_query3(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let mut ind_count = vec![0; 100000];
    for v in &requests {
        for ind in v[0]..=v[1] {
            ind_count[ind as usize] += 1
        }
    }

    ind_count.sort();
    ind_count.reverse();

    let mut sort_nums = nums.clone();
    sort_nums.sort();
    sort_nums.reverse(); // larger to smaller

    let mut module_table: HashMap<i32, i32> = HashMap::new();

    let mut result = 0;
    for ind in 0..nums.len() {
        //result = result % 1000000007 + (ind_count[ind] * sort_nums[ind]) % 1000000007;
        result = result % 1000000007
            + get_big_module(&mut module_table, ind_count[ind] * sort_nums[ind]);
    }

    result
}

fn get_big_module(table: &mut HashMap<i32, i32>, num: i32) -> i32 {
    if num < 1000000007 {
        return num;
    }

    match table.get(&num).as_ref() {
        Some(n) => {
            return **n;
        }
        None => {
            let aa = (1 + get_big_module(table, num - 1)) % 1000000007;
            table.insert(num, aa);
        }
    }

    *table.get(&num).unwrap()
}

pub fn max_sum_range_query4(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    ///////////////////////////
    // this part calculate each index's count;
    // very enlighting way
    let mut count_vec = vec![0_i64; nums.len() + 1];
    for v in &requests {
        count_vec[v[0] as usize] += 1;
        count_vec[v[1] as usize + 1] -= 1;
    }

    for i in 1..=nums.len() {
        count_vec[i] += count_vec[i - 1]
    }
    dbg!(&count_vec);
    /////////////////////////

    let mut result = 0;

    nums.sort();
    count_vec = count_vec.drain(..count_vec.len() - 1).collect();
    count_vec.sort();

    for i in 0..nums.len() {
        result += (count_vec[i] * nums[i] as i64) % 1000000007;
        result = result % 1000000007
    }
    result as i32
}

fn main() {
    assert_eq!(
        max_sum_range_query4(
            vec![1, 2, 3, 4, 5, 10],
            vec![vec![0, 2], vec![1, 3], vec![1, 1]]
        ),
        47
    );

    assert_eq!(
        max_sum_range_query4(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1]]),
        11
    );

    assert_eq!(
        max_sum_range_query4(vec![1, 2, 3, 4, 5], vec![vec![1, 3], vec![0, 1]]),
        19
    );
}
