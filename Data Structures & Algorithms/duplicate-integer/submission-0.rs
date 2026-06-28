impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique_nums = HashSet::new();

        for num in &nums {
            if !unique_nums.insert(num) {
                return true;
            }
        }
        
        false
    }
}
