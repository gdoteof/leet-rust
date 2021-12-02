use std::collections::HashMap;
use std::convert::TryFrom;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{

    //From values to indexes 
    let mut vals:HashMap<i32,i32> = HashMap::new();

    for (i, x) in nums.iter().enumerate() {
        let i_actual = i32::try_from(i).unwrap();
        let needed = target - x;
        match vals.get(&needed) {
            Some(other) => return vec![i_actual, *other],
            None => { vals.insert(*x, i_actual); } 
        }
    }

    return nums;
}

pub fn _two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32>{
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if j <= i { continue; }

            if x+y == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return nums;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testit(expected: Vec<i32>, actual: Vec<i32>){
        let (e, a) = (expected.clone().sort(), actual.clone().sort());
        assert_eq!(e,a);
    }

    #[test]
    fn it_works() {
        testit(two_sum(vec![1,2,3], 5), vec![1,2]);
    }

}
