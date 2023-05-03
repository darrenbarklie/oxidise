fn main() {
    // Iterate over a Vector and return modified values
    // let list: Vec<_> = vec![1, 2, 3].iter().map(|i| return i + 1).collect();
    // println!("Result: {:?}", list);

    // Manually demostrate the .collect() method
    let data = vec![1, 2, 3];
    let mut list = data.iter().map(|x| x + 1);
    let mut new_vector = vec![];
    while let Some(x) = list.next() {
        new_vector.push(x)
    }
    println!("Result: {:?}", new_vector);
}
