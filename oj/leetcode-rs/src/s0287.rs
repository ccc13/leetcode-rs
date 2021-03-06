pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Floyd's Tortoise and Hare

        let mut tortoise = nums[0] as usize;
        let mut hare = nums[0] as usize;
        loop {
            tortoise = nums[tortoise] as usize;
            hare = nums[nums[hare] as usize] as usize;

            if tortoise == hare {
                break;
            }
        }

        let mut ptr1 = nums[0] as usize;
        let mut ptr2 = tortoise;

        while ptr1 != ptr2 {
            ptr1 = nums[ptr1] as usize;
            ptr2 = nums[ptr2] as usize;
        }

        ptr1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
