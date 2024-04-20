# Sortinglab
Rust Sorting Library
This is a library providing functions for sorting various data types in the Rust programming language. It includes implementations of sorting algorithms such as quicksort, selection sort, insertion sort, and merge sort.

# Installation
To use this library, add it to the dependencies of your project in the Cargo.toml file:

toml
Copy code
[dependencies]
sorting_library = "0.1.0"
Usage
Example of using the quicksort function for a vector of integers:

rust
Copy code
use sorting_library::{quick_sort, Sortable};

'''let mut numbers = vec![5, 3, 7, 1, 9];'''
quick_sort(&mut numbers);
println!("Sorted vector: {:?}", numbers);
You can also define your own data types by implementing the Sortable trait and sort collections of objects of your type.

rust
Copy code
use sorting_library::{quick_sort, Sortable};

// Define a structure implementing the Sortable trait
struct MyObject {
    value: i32,
}

impl Sortable for MyObject {
    fn compare(&self, other: &Self) -> bool {
        self.value < other.value
    }
}

// Example usage
let mut objects = vec![
    MyObject { value: 5 },
    MyObject { value: 3 },
    MyObject { value: 7 },
];

quick_sort(&mut objects);
Sorting Algorithms
The following sorting algorithms are implemented in our library:

Quicksort (quick_sort)
Selection sort (selection_sort)
Insertion sort (insertion_sort)
Merge sort (merge_sort)
Requirements
This library requires Rust version 1.0 or above.

License
This project is licensed under the MIT License. See the LICENSE file for details.
