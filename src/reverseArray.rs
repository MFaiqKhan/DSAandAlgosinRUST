// code type 1 :

fn reverseArray(a: &[i32]) -> Vec<i32> {
    let mut size = a.len();
    let mut count = 1;
    let mut b:Vec<i32> = Vec::new();
    while count <= size {
        b.push(a[a.len() - count]);
        count+=1;
    };
    b
    
}

// code type 2 :


fn reverseArray(a: &[i32]) -> Vec<i32> {
    let mut b: Vec<i32> = Vec::new();
    let size = a.len();
    for i in 0..size {          // 0..size is basically telling the for loop to iterate from 0 to size
        b.push(a[size - i - 1]);
    }
    b
}
