fn optimizedArrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut array:Vec<i64> = vec![0; (n+1) as usize]; // create a vector of size n with all elements 0, 1-indexed array
    // array will be : [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] if n = 10 (11 elements) as we are adding 1 to n
    let mut max = 0; // create a variable to store the maximum element in the array
    for query in queries { // iterate over the queries, each query is a vector of 3 elements
        let a = query[0] as usize; // convert the first element of the query to a usize, name it a
        let b = query[1] as usize; // convert the second element of the query to a usize, name it b
        let k = query[2]; // get the third element of the query, name it k this is the value to add to the array
        array[a-1] += k as i64; // add k to the element at index a-1, for example if a = 1, b = 3, and k = 100, then we add 100 to array[1-1] which is array[0]
        if b < array.len() { // if b is less than the length of the array, then we need to subtract k from array[b]
            array[b] -= k as i64; // subtract k from the element at index b, for example if a = 1, b = 3, and k = 100, then we subtract 100 from array[3]
        }
    };
    // get the maximum element in the array
    let mut sum = 0; // create a variable to store the sum of the elements in the array
    for i in 0..array.len() { // iterate over the array
        sum += array[i]; // add the element at index i to the sum
        max = if sum > max {sum} else {max}; // if the sum is greater than max, then set max to the sum, else keep max the same
    }
    max as i64 // return the max
}

// prefix sum array solve in c++ // https://medium.com/@mlgerardvla/hackerrank-array-manipulation-beat-the-clock-using-prefix-sum-92471060035e

// dry run for the optimizedArrayManipulation function
// array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// queries = [[1, 5, 3], [4, 8, 7], [6, 9, 1]]
// first query: a = 1, b = 5, k = 3
// array[1-1] += k, array[0] += 3
// array = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// if b < array.len() {array[b] -= k}, if 5 < 11 {array[5] -= 3}
// array = [3, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0]
// second query: a = 4, b = 8, k = 7
// array[4-1] += k, array[3] += 7
// array = [3, 0, 0, 7, 0, -3, 0, 0, 0, 0, 0]
// if b < array.len() {array[b] -= k}, if 8 < 11 {array[8] -= 7}
// array = [3, 0, 0, 7, 0, -3, 0, 0, -7, 0, 0]
// third query: a = 6, b = 9, k = 1
// array[6-1] += k, array[5] += 1
// array = [3, 0, 0, 7, 0, -2, 0, 0, -7, 0, 0]
// if b < array.len() {array[b] -= k}, if 9 < 11 {array[9] -= 1}
// array = [3, 0, 0, 7, 0, -2, 0, 0, -7, -1, 0]
// sum = 0, max = 0
// i = 0, sum += array[0], sum = 3, max = 3
// i = 1, sum += array[1], sum = 3, max = 3
// i = 2, sum += array[2], sum = 3, max = 3
// i = 3, sum += array[3], sum = 10, max = 10
// i = 4, sum += array[4], sum = 10, max = 10
// i = 5, sum += array[5], sum = 8, max = 10
// i = 6, sum += array[6], sum = 8, max = 10
// i = 7, sum += array[7], sum = 8, max = 10
// i = 8, sum += array[8], sum = 1, max = 10
// i = 9, sum += array[9], sum = 0, max = 10
// i = 10, sum += array[10], sum = 0, max = 10
// max = 10



/* 
This for loop is used to create a prefix sum array, which is a way of efficiently storing the sum of elements in a
range in an array.
The prefix sum array is created by storing the difference between two elements in the
array instead of the actual element.
For example, let's say we have an array [1, 3, 6, 10, 15], and we want 
to create a prefix sum array for it.
The prefix sum array for this array would be [1, 2, 3, 4, 5],
because the difference between each element in the prefix sum array is the value 
of the element in the original array.
So, the prefix sum array allows us to easily find the sum of
elements in a range by simply adding up the elements in the prefix sum array that fall within that range.
For example, if we want to find the sum of elements in the range [2, 4] 
in the original array, we can simply add up the elements in the prefix

*/




fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
 let mut array:Vec<i32> = vec![0; (n+1) as usize]; // create a vector of size n with all elements 0, 1-indexed array
 // array will be : [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] if n = 10 (11 elements) as we are adding 1 to n
 let mut max = 0; // create a variable to store the maximum element in the array
 for query in queries { // iterate over the queries, each query is a vector of 3 elements
    let a = query[0] as usize; // convert the first element of the query to a usize, name it a
    let b = query[1] as usize; // convert the second element of the query to a usize, name it b
    let k = query[2]; // get the third element of the query, name it k this is the value to add to the array
    for i in a-1..=b-1 { // iterate over the array from a to b, inclusively means we need to use ..= instead of ..
        // as array is 1-indexed,  we need to subtract 1 from a and b to get the correct index
        array[i] += k; // add k to the element at index i, for example if a = 1, b = 3, and k = 100, then we add 100 to array[1], array[2], and array[3]
        max = if array[i] > max {array[i]} else {max}; // if the element at index i is greater than max, then set max to that element, else keep max the same
    };
    }
    max as i64 // return the max

}



/* 
In the given code, a and b represent the starting and ending indices, 
respectively, of a subarray in the array vector. The indices in Rust start from 0, 
so if a and b are given as 2 and 6, respectively, we need to iterate over the elements of array from
index 1 (which is a-1) to index 5 (which is b-1).
The reason for this is that in Rust, the range a..=b includes both a and b, so if we write for i in a..=b, the 
i will take values from a to b inclusively. Therefore, if we want to iterate over the 
elements of array from index a to index b (both inclusive), we need to write for i in a-1..=b-1.
This is because the first index of the array vector is 0, so a-1 will be the index from which we want to start the 
iteration, and b-1 will be the index where we want to end the iteration.
For example, if a is 2 and b is 6, then a-1 is 1 and b-1 is 5. So the loop for i in a-1..=b-1 will 
iterate over the elements of array from index 1 to index 5, both inclusive.
*/

// for every query in the queries vector, we iterate over the array from a to b, and add k to each element in the array
// then we return the maximum element in the array

// vectors have a method called with_capacity which takes in a usize and returns a vector of that size