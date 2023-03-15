/**
 * 使用Rust语言写一个冒泡排序的算法
 * 基础要求：固定类型（比如i32）的数组排序
 * 提高部分：能够使用template和PartialOrd实现对任意类型的排序
 */
fn main() {
    // 基础
    let mut arr = [9, 3, 2, 1, 7, 0];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
    // 泛型
    let mut arr = ["java", "c", "python", "rust", "js", "go", "0"];
    bubble_sort_generic(&mut arr);
    println!("{:?}", arr);
}
/**
 * 基础要求：固定类型（比如i32）的数组排序
 */
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
/**
 * 提高部分：能够使用template和PartialOrd实现对任意类型的排序
 */
fn bubble_sort_generic<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
