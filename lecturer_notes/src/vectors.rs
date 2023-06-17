#![allow(dead_code)]
/*
There are many methods for Vectors in Rust that are similar to
those for arrays in other programming languages. Let's take a look
at some of them.

pub fn dedup(&mut self):

    Removes consecutive repeated elements in the vector
    according to the PartialEq trait implementation.
    If the vector is sorted, this removes all duplicates.

pub fn sort(&mut self):

    Sorts the vector in-place using the Ord trait implementation,
    an adaptive, iterative merge sort algorithm which is fast
    for almost sorted data that is also stable and not recursive.

pub fn sort_by<F>(&mut self, compare: F):

    Sorts the vector in-place by the given comparison function.

pub fn sort_unstable(&mut self):

    Sorts the slice, but might not preserve the order of equal elements.
    This sort is unstable (i.e., may reorder equal elements), in-place
    (i.e., does not allocate), and O(n * log(n)) worst-case.

pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>:

    Returns a reference to an element or subslice depending on the type of index.
    If given a position, returns a reference to the element at that position
    or None if out of bounds.
    If given a range, returns the subslice corresponding to that range, or
    None if out of bounds.

pub fn insert(&mut self, index: usize, element: T):

    Inserts an element at position index within the vector, shifting all elements
    after it to the right.

pub fn is_empty(&self) -> bool:

    Returns true if the vector contains no elements.

pub fn retain<F>(&mut self, f: F):

    Retains only the elements specified by the predicate.

*/

pub fn main() {
    // First, let's declare a mutable vector:
    let mut vec = vec![1, 2, 2, 3, 3, 3];

    // Using dedup to remove consecutive repeated elements:
    vec.dedup();
    assert_eq!(vec, [1, 2, 3]);

    // Let's reset the vector and try some other methods:
    vec = vec![3, 1, 2];

    // Using sort to sort the vector:
    vec.sort();
    assert_eq!(vec, [1, 2, 3]);

    // Let's reset the vector and try some other methods:
    vec = vec![3, 1, 2];

    // Using sort_unstable to sort the vector:
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);

    // Using get to get the element at a certain index:
    assert_eq!(vec.get(1), Some(&2));
    assert_eq!(vec.get(10), None);

    // Using insert to insert an element at a certain position:
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);

    // Using is_empty to check if the vector is empty:
    assert_eq!(vec.is_empty(), false);

    vec.clear();
    assert_eq!(vec.is_empty(), true);

    // Using is_empty to check if the vector is empty:
    let mut nums = vec![1, 2, 3, 4];

    match nums.is_empty() {
        true => println!("No numbers found!"),
        false => println!("There are {} numbers in the vector!", nums.len()),
    }
    // Now, let's empty the vector:
    nums.clear();

    // Should be empty this time around! 🙃
    match nums.is_empty() {
        true => println!("No numbers found!"),
        false => println!("There are {} numbers in the vector!", nums.len()),
    }

    // Retain vector method:
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Vector before retain: {:?}", numbers);  // Output: [1, 2, 3, 4, 5]

    // Retain only even numbers
    numbers.retain(|&x| x % 2 == 0);

    println!("Vector after retain: {:?}", numbers);  // Output: [2, 4]

    /*
    Shorthand Vector Initializers:
    In Rust, you can use shorthand syntax to initialize vectors with repetitive values.
    For example, to create a vector of length 5 with the value 0 repeated in each element,
    you can use the following shorthand syntax:
    */

    let vec1 = vec![0; 5];
    print!("vec: {:?}", vec1);
    let vec2 = vec!["hello"; 3];
    print!("vec: {:?}", vec2);

}
