fn main() {
    println!("Hello, world!");
    let arr: Vec<i32> = vec![1, 2, 3,4,8,12,15];
    let target: i32 = 9;
    let index: i32 = binary_search(&arr, target);
    if index > 0 {
        println!("found target {} at {}", target, index)
    }else {
        println!("target {} not found", target)
    }
}

fn binary_search(arr : &Vec<i32>, target: i32) -> i32 {
    let mut low: usize = 0;
    let mut high: usize = arr.len();

    while low <= high {
        let mid: usize = low+(high-low)/2;
        if arr[mid] > target {
            high = mid-1;
        }else if arr[mid] < target {
            low = mid+1;
        }else {
            return mid as i32;
        }
    }

    return -1;
}

