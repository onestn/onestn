fn main() {
    let vec1 = Vec::from([1, 2, 3]);
    let vec2 = vec![1, 2, 3];

    // empty vectors
    let vec3: Vec<i32> = Vec::new();
    let vec4: Vec<i32> = vec![];

    let num = vec1[1];
    println!("{}", num);

    let mut mut_vec1 = vec![1, 2, 3];
    mut_vec1.push(4);
    mut_vec1.push(5);
    mut_vec1.push(6);

    println!("{:?}", mut_vec1);
}
