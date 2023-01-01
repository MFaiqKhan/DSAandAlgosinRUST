


// how hourglass looks like in array 
// 1 1 1 
//   1  
// 1 1 1 



pub fn array2d_hourglass(arr: &[Vec<i32>]) -> i32{
    let mut max = -100; // -100 is the lowest possible sum of an hourglass, we gonna compare it to the sum of each hourglass
    for i in 0..arr.len() { // Vector.len() is the number of rows in the array
        for j in 0..arr.len() { // Vector.len() is the number of columns in the array
            if i + 2 < arr.len() && j+2 < arr.len() { // if the hourglass in inside the array
                // sum = array[][] + array[][] + array[][] + array[][] + array[][] + array[][] + array[][]; // sum of the hourglass, total of 7 elements
                let mut sum = arr[i][j] + arr[i][j+1] + arr[i][j+2] + arr[i+1][j+1] + arr[i+2][j] + arr[i+2][j+1] + arr[i+2][j+2];  // output of the hourglass
                if sum > max { // if the sum of the hourglass is bigger than the max
                    max = sum; // max = sum
                }
            }
        }
    }
    return max;
}

// dry run the above code with this array
// 1 1 1 0 0 0
// 0 1 0 0 0 0
// 1 1 1 0 0 0
// 0 0 2 4 4 0
// 0 0 0 2 0 0

// i = 0, j = 0
// sum = 1 + 1 + 1 + 1 + 1 + 1 + 1 = 7
// max = 7

// i = 0, j = 1
// sum = 1 + 1 + 0 + 1 + 1 + 0 + 0 = 4
// max = 7

// i = 0, j = 2
// sum = 1 + 0 + 0 + 0 + 1 + 0 + 0 = 2
// max = 7








// &[Vec<i32>] is a reference to a vector of vectors of i32/
// &[Vec<i32>] returns a reference to a vector of vectors of i32/

// why do we need to use a reference to a vector of vectors of i32?
// because we are not going to modify the array, we are just going to read it, so we can use a reference to it, so we don't have to copy the whole array to the function, we just copy the reference to it, so we can save memory and time.

// why we need i+2 < arr.len() and j+2 < arr.len()? and not i+2 <= arr.len() and j+2 <= arr.len()? 
// because we need to check if the hourglass is inside the array, if we use i+2 <= arr.len() and j+2 <= arr.len() we are going to check if the hourglass is outside the array, and we don't want that, we want to check if the hourglass is inside the array.
// e.g In the last row of the array, the hourglass is going to be like this:
// 0 0 0
//   0
// 0 0 0
// so here if i and j will be 3, and if we use i+2 <= arr.len() and j+2 <= arr.len() we are going to check if the hourglass is outside the array, and we don't want that, we want to check if the hourglass is inside the array.
// and third index doesn't exist in the array, so we can't use it, so we need to use i+2 < arr.len() and j+2 < arr.len().