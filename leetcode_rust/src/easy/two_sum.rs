use std::collections::btree_map::BTreeMap;

pub fn two_sum_test() {
    let nums = [2,7,11,15];
    let res = two_sum(nums.to_vec(), 9);
    println!("res: {:?}", res);

    let res = two_sum_btree_map(nums.to_vec(), 9);
    println!("res: {:?}", res);

    let res = two_sum_btree_map_optimize(nums.to_vec(), 9);
    println!("res: {:?}", res);
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    for i in 0..nums.len() {
        let temp = target - nums[i];
        for j in i+1..nums.len() {
            if temp == nums[j] {
                res.push(i as i32);
                res.push(j as i32);
                return res;
            }
        }
    }
    return res
}


pub fn two_sum_btree_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut bmap = BTreeMap::new();
    for i in 0..nums.len() {
        bmap.insert(nums[i], i);
    }
    let mut res = Vec::new();
    for i in 0..nums.len() {
        let temp = target - nums[i];
        if bmap.contains_key(&temp) {
            res.push(i as i32);
            res.push(*bmap.get(&temp).unwrap() as i32);
            return res;
        }
    }
    return res;
}

pub fn two_sum_btree_map_optimize(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut bmap = BTreeMap::<i32, i32>::new();
    let mut res = Vec::<i32>::new();
    for i in 0..nums.len() {
        let temp = target - nums[i];
        if bmap.contains_key(&temp) {
            res.push(*bmap.get(&temp).unwrap());
            res.push(i as i32);
            return res;
        }
        bmap.insert(nums[i], i as i32);
    }
    return res;
}