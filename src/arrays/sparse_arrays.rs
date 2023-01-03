

fn matchingStrings(stringList: &[String], queries: &[String]) -> Vec<i32> {
    let mut result:Vec<i32> = Vec::new();  // create a new vector to store the result  
    for query in queries { // for every query in the queries array
        let mut count = 0;  // create a counter to count the number of matches
        for string in stringList { // for every string in the stringList array
            if query == string { // query searches the stringlist for matches , if there is a match, the counter is incremented
                count += 1; // increment the counter
            }
        }
        result.push(count); // push the count to the result vector
        // so basically here we are pushing the count of the number of matches to the result vector, and then we are going to the next query
    } 
    result
}   