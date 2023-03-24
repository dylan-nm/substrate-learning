// 定义一个函数，为u32类型的整数集合求和
fn sum_u32(nums: &[u32]) -> Option<u32> {
    // 使用iter方法和try_fold方法求和，并使用checked_add方法避免溢出
    nums.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

fn main() {
    // 定义一个包含一些数字的u32类型的整数集合
    let nums = &[1, 2, 4294967295];

    // 调用sum_u32函数求和，并打印结果
    match sum_u32(nums) {
        Some(s) => println!("Sum is {}", s),
        None => println!("Overflow occurred"),
    }
}
