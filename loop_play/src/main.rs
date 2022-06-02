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

// 2d array

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let le1 = matrix.len();
    let le2 = matrix[0].len();
    let mut new_arr = vec![vec![0; le1]; le2];
    for i in 0..le1 {
        for j in 0..le2 {
            new_arr[j][i] = matrix[i][j]
        }
    }
    return new_arr;
}

pub fn transpose2(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = Vec::new();
    for i in 0..matrix.len() {
        
        for j in 0..matrix[0].len() {
            if i == 0 {
                let mut cur:Vec<i32> = Vec::new();
                res.push(cur);
            }
            
            res[j].push(matrix[i][j]);
        }
        
    }
    res
}