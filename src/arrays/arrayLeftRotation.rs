
fn array_rotate_left(d: i32, arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec(); // converting the array to a vector
    let size = result.len(); // getting the size of the vector
    for _ in 0..d { // _ is used to ignore the value of the iterator, it iterates from 0 to d, if d is 2, it will iterate 2 times
        let temp = result[0]; // storing the first element in a variable
        for i in 0..size -1 { // iterating through the array from 0 to size - 1 which is the last index of the array
            result[i] = result[i + 1]; // shifting the elements to the left
        }
        result[size - 1] = temp; // putting the first element at the end
    }
    result
}

//explain above code :
 // the function takes two arguments, d and arr, d is the number of rotations and arr is the array to be rotated
    // the function returns a vector, which is the rotated array
    // the function first converts the array to a vector, and stores it in a variable called result
    // the function then gets the size of the vector and stores it in a variable called size

