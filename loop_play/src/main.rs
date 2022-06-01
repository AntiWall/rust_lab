fn main() {
    
}

// iterator
fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for i in v.iter() {
        res.push(i + n);
    }
    res
}
// mut iterator
fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for item in v.iter_mut(){
        *item += n;
    }
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.push(nums[0]);
    for i in 1..nums.len() {
        v.push(v[i - 1] + nums[i]);
    }
    v
}