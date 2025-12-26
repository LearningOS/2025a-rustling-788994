/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
//

fn sort<T:Ord>(array: &mut [T]){
	//TODO
    // 如果数组长度 <= 1，直接返回（已经有序）
    if array.len() <= 1 {
        return;
    }

    let mut swapped;
    let mut n = array.len();

    // 冒泡排序核心逻辑
    loop {
        swapped = false;
        // 遍历未排序部分，比较相邻元素
        for i in 1..n {
            // 如果前一个元素大于后一个，交换位置
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped = true;
            }
        }

        // 每轮结束后，最后一个元素已排序，缩小遍历范围
        n -= 1;

        // 如果本轮没有交换，说明数组已完全有序，退出循环
        if !swapped {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}