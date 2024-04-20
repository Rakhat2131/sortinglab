// Define a trait for sortable objects
pub trait Sortable {
    fn compare(&self, other: &Self) -> bool;
}

// Define the sorting algorithms
pub mod sorting {
    use crate::Sortable;

    // Selection sort implementation
    pub fn selection_sort<T: Sortable>(arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_idx = i;
            for j in (i + 1)..arr.len() {
                if arr[j].compare(&arr[min_idx]) {
                    min_idx = j;
                }
            }
            if min_idx != i {
                arr.swap(i, min_idx);
            }
        }
    }

    // Insertion sort implementation
    pub fn insertion_sort<T: Sortable>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && !arr[j].compare(&arr[j - 1]) {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    // Quick sort implementation
    pub fn quick_sort<T: Sortable>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        quick_sort(left);
        quick_sort(&mut right[1..]);
    }

    // Partition function for quick sort
    fn partition<T: Sortable>(arr: &mut [T]) -> usize {
        let pivot_index = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot_index {
            if arr[j].compare(&arr[pivot_index]) {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_index);
        i
    }

    // Merge sort implementation (remaining unchanged)
    pub fn merge_sort<T: Sortable + Copy>(arr: &mut [T]) {
        if arr.len() < 2 {
            return;
        }

        let len = arr.len();
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        let mut tmp = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = mid;

        while i < mid && j < len {
            if arr[i].compare(&arr[j]) {
                tmp.push(arr[i]);
                i += 1;
            } else {
                tmp.push(arr[j]);
                j += 1;
            }
        }

        if i < mid {
            tmp.extend_from_slice(&arr[i..mid]);
        } else if j < len {
            tmp.extend_from_slice(&arr[j..len]);
        }

        arr.copy_from_slice(&tmp[..]);
    }
}

// Implement sorting functions for any type that implements Sortable
impl<T: PartialOrd> Sortable for T {
    fn compare(&self, other: &Self) -> bool {
        *self < *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut cities = vec!["New York", "Los Angeles", "Chicago", "Houston"];
        sorting::selection_sort(&mut cities);
        assert_eq!(cities, vec!["Chicago", "Houston", "Los Angeles", "New York"]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut cities = vec!["New York", "Los Angeles", "Chicago", "Houston"];
        sorting::insertion_sort(&mut cities);
        assert_eq!(cities, vec!["Chicago", "Houston", "Los Angeles", "New York"]);
    }

    #[test]
    fn test_quick_sort() {
        let mut numbers = vec![9, 5, 2, 7, 3, 1, 8, 4, 6];
        sorting::quick_sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut floats = vec![3.5, 1.2, 2.8, 0.5, 5.9];
        sorting::merge_sort(&mut floats);
        assert_eq!(floats, vec![0.5, 1.2, 2.8, 3.5, 5.9]);
    }
}
