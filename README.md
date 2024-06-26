# Sortinglab
Rust Sorting Library
This library provides functions for sorting various data types in the Rust programming language. It includes implementations of sorting algorithms such as quicksort, selection sort, insertion sort, and merge sort.

Sorting algorithms are fundamental in programming and find applications in a multitude of tasks, including organizing data for more efficient searching, processing, and analysis. Using this library allows developers to quickly and easily incorporate sorting into their Rust projects, ensuring optimal performance and ease of use.

With various algorithms provided by this library, developers can choose the most suitable sorting method based on the specific requirements and characteristics of their data. The library also offers flexibility in usage thanks to its generic implementations, allowing sorting of different types of data with ease.

# Installation
To use this library, add it to the dependencies of your project in the Cargo.toml file:

toml
Copy code
`
[dependencies]
sorting_library = "https://github.com/Rakhat2131/sortinglab"
`
<img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-51-21.png><br>
# Usage
Example of using the quicksort function for a vector of integers:

rust
Copy code
use sorting_library::{quick_sort, Sortable};

let mut numbers = vec![5, 3, 7, 1, 9];
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

# Example usage
`
let mut objects = vec![
    MyObject { value: 5 },
    MyObject { value: 3 },
    MyObject { value: 7 },
];
quick_sort(&mut objects);
`
Sorting Algorithms
The following sorting algorithms are implemented in our library:

Quicksort (quick_sort)
Selection sort (selection_sort)
Insertion sort (insertion_sort)
Merge sort (merge_sort)
<img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-38-48.png><br>
<img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-38-58.png<br>
<img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-39-03.png<br>
# Requirements
This library requires Rust version 1.0 or above.
# Tests
 <img src=https://github.com/Rakhat2131/sortinglab/raw/main/images/2024-04-20_23-36-00.png><br>
 <img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-36-11.png><br>
 <img src=https://github.com/Rakhat2131/sortinglab/blob/main/images/2024-04-20_23-38-36.png><br>
# License
This project is licensed under the MIT License. See the LICENSE file for details.
