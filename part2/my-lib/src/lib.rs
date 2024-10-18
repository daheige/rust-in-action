// 求一个包含i32类型元素的切片中最大值
pub fn max(nums: &[i32]) -> i32 {
    let mut max_num = nums[0];
    for i in nums.to_vec() {
        if i > max_num {
            max_num = i;
        }
    }

    max_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = vec![89, 2, 23, 12, 89, 23, 87];
        let m = max(&s);
        println!("max(s) = {}", m);
        assert_eq!(max(&[1, 2, 3, 5, 9, 7]), 9);
    }
}
