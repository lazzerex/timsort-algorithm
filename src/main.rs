fn insertion_sort(arr: &mut [i32], start: usize, end: usize) {
    for i in start + 1..end + 1 {
        let mut j = i;
        while j > start && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize) {
    let left = arr[start..=mid].to_vec();
    let right = arr[mid + 1..=end].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn timsort(arr: &mut [i32]) {
    let run_size = 32;

    for i in (0..arr.len()).step_by(run_size) {
        insertion_sort(arr, i, std::cmp::min(i + run_size - 1, arr.len() - 1));
    }

    let mut size = run_size;
    while size < arr.len() {
        for start in (0..arr.len()).step_by(2 * size) {
            let mid = start + size - 1;
            let end = std::cmp::min(start + 2 * size - 1, arr.len() - 1);
            if mid < end {
                merge(arr, start, mid, end);
            }
        }
        size *= 2;
    }
}

fn main() {
    //let mut data = [13, 2, 4, 0, 5, 1, 23, 3, 6, 9 ,10, -1, 45, 5];
	//let mut data: [i32; 30]  = rand::random(); 
    //timsort(&mut data);
    //println!("{:?}", data); // Output: [1, 2, 3, 4, 5, 6, 7, 8, 9]
	
	
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // Fix 1: Use ..= for an inclusive range in Vec creation
    // Fix 2: Use gen_range() with proper range syntax
    let mut data: Vec<i32> = (0..=30).map(|_| rng.gen_range(-100..=100)).collect();
    timsort(&mut data);
    println!("{:?}", data);

}

//------------------Testing goes here----------------------------//

#[cfg(test)]

mod tests {
	use super::*;
	use std::iter;
	
	/*#[test]
	fn test_empty_array() {
			let mut arr: Vec<i32> = vec![];
			timsort(&mut arr);
			assert_eq!(arr, vec![1]);
	}*/
	
	#[test]
	fn test_single_element() {
		let mut arr = vec![1];
		timsort(&mut arr);
		assert_eq!(arr, vec![1]);
	}
	
	#[test]
	
	fn test_already_sorted() {
		let mut arr = vec![1, 2, 3, 4, 5];
		timsort(&mut arr);
		assert_eq!(arr, vec![1, 2, 3, 4, 5]);
	}
	
	#[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        timsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicate_elements() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, 2, -8, 10, -3, 0, 7, -1];
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_random_array() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = (0..100).map(|_| rng.gen_range(-100..100)).collect();
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_all_same_elements() {
        let mut arr: Vec<i32> = iter::repeat(42).take(100).collect();
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_alternating_elements() {
        let mut arr = vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let mut expected = arr.clone();
        expected.sort();
        timsort(&mut arr);
        assert_eq!(arr, expected);
    }

    

    #[test]
    fn test_merge() {
        let mut arr = vec![1, 3, 5, 2, 4, 6];
        merge(&mut arr, 0, 2, 5);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}




















