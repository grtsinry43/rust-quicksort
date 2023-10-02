// 交换数组中两个元素的值
fn swap(slice: &mut [i32], i: usize, j: usize) {
    let temp = slice[i];
    slice[i] = slice[j];
    slice[j] = temp;
}

// 对数组进行分区，选择一个基准元素，并返回其索引
fn partition(slice: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = slice[end]; // 选择最后一个元素作为基准
    let mut i = start; // i指向第一个小于等于基准的元素
    let mut j = start; // j指向当前遍历的元素
    while j < end {
        if slice[j] <= pivot {
            swap(slice, i, j); // 如果当前元素小于等于基准，就交换它和i指向的元素，并将i后移一位
            i += 1;
        }
        j += 1;
    }
    swap(slice, i, end); // 最后，将基准元素和i指向的元素交换，使得基准元素在正确的位置上
    i // 返回基准元素的索引
}

// 对数组进行快速排序
fn quick_sort(slice: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return; // 如果数组为空或只有一个元素，直接返回
    }
    let p = partition(slice, start, end); // 对数组进行分区，并得到基准元素的索引
    quick_sort(slice, start, p - 1); // 对左边的子数组进行快速排序
    quick_sort(slice, p + 1, end); // 对右边的子数组进行快速排序
}

// 主函数，用于测试快速排序算法
fn main() {
    let mut arr = vec![10, 7, 8, 9, 1, 5]; // 定义一个待排序的数组
    println!("原始数组：{:?}", arr); // 打印原始数组
    let n = arr.len(); // 获取数组长度
    quick_sort(&mut arr, 0, n - 1); // 调用快速排序函数，传入数组、起始索引和终止索引
    println!("排序后的数组：{:?}", arr); // 打印排序后的数组
}
