#![allow(dead_code)]


fn dynamic_array(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
    let mut sequence:Vec<Vec<i32>> = vec![Vec::new(); n as usize]; // here n is the number of sequences means the number of vectors in the vector b
    let mut last_answer:i32 = 0; // this is the last answer that will be returned
    let mut result: Vec<i32> = Vec::new(); // this is the vector that will be returned

    // Queries are 3 space separated integers, 1 x y, 2 x y	
    // here 1 and 2 are representing the type of query
    //1 x y means 1st type of query and x and y are the values of the query
    // 2 x y means 2nd type of query and x and y are the values of the query

    // e.g: 1 0 5
    // 1 is the type of query
    // 0 is the index of the sequence which will inserted in formula, 0 is acting as  x
    // 5 is the value to be inserted in the sequence, 5 is acting as  y

    // formula for finding the sequence number is (x ^ last_answer) % n

    // In query 1, we append the value y to the sequence at index ((x ^ last_answer) % n).	
    // In query 2, we find the sequence at index ((x ^ last_answer) % n) and assign the value at index y % size to last_answer.


    // we could be given unlimted number of queries but they will contain only 3 elements and two types of queries.

    for i in 0..queries.len() { // here queries.len() is the number of queries, i is the index of the query
        let seq_index = (queries[i][1] ^ last_answer) % n; // here 1 is the index of x in the query
        let _x = queries[i][1]; 
        let y = queries[i][2]; // y is the value to be inserted in the sequence

        if(queries[i][0]) == 1 { // here 0 is the index of the type of query, if 1 then it is the first type of query
            sequence[seq_index as usize].push(y);
        }
        else if(queries[i][0]) == 2 { // if it is 2 then it is the second type of query
            last_answer = sequence[seq_index as usize][(y as usize % sequence[seq_index as usize].len()) as usize]; //  here sequence[seqIndex].len() is the size of the sequence
            result.push(last_answer);
        }
    }

    return result;
    

}



// (y as usize % sequence[seqIndex as usize].len()) as usize , the uszie is used to convert the value to the usize type
// we have to use usize at the end because the result of the expression is usize type, otherwise it will give an error

// line 6:
/* 
In this code, sequence is a variable that is being declared as a vector of vectors of i32 values. 
The vec! macro is used to create a new vector, and the Vec::new method is used to create new vectors to be 
inserted into the outer vector.
The vec! macro creates a new vector with the specified elements.
In this case, the vector is being created with no elements, because Vec::new returns an empty vector.
The n as usize expression converts the value of n to a usize type, which is required by the vec! macro.
The usize type is an unsigned integer type that is used to represent the size of a value in memory.
The resulting vector, sequence, will be a vector with n elements, 
where each element is an empty vector of i32 values. 
*/