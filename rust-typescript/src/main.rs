use std::collections::{HashMap, HashSet};

fn main() {
    // Iterate over a Vector and return modified values
    let list: Vec<_> = vec![1, 2, 3].iter().map(|i| return i + 1).collect();
    println!("Result 1: {:?}", list);

    // Manually demostrate the .collect() method
    let data = vec![1, 2, 3];
    let mut list = data.iter().map(|x| x + 2);
    let mut new_vector = vec![];
    while let Some(x) = list.next() {
        new_vector.push(x)
    }
    println!("Result 2: {:?}", new_vector);

    // By stating type String, collect() modifies behaviour to concat
    let data: String = vec!["this", "is", "a", "call"].into_iter().collect();
    println!("Result 3: {:?}", data);

    // Collect into a HashSet
    let data: HashSet<isize> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
    println!("Result 4: {:?}", data);

    // Collect into a HashMap, destructuring the response
    let data: HashMap<&str, usize> = vec!["we", "will", "we", "will", "rock", "you"]
        .into_iter()
        .enumerate()
        .map(|(idx, item)| (item, idx))
        .collect();
    println!("Result 5: {:?}", data);

    // Sum
    let value: usize = vec![1, 2, 3].iter().sum();
    println!("Result 6: {:?}", value);

    // Count counts how many times it can call next() on iter()
    let value: usize = vec![1, 2, 3, 4, 5].iter().skip(3).count();
    println!("Result 7: {:?}", value);

    // Pattern matching
    vec![1, 2, 5, 9, 4, 8]
        .iter()
        .skip(2)
        .take_while(|&&x| x > 4)
        .for_each(|x| println!("Result 8: {}", x));

    // Test for odd/even
    let value: usize = vec![1, 2, 3, 4, 5].iter().filter(|x| *x % 2 == 0).count();
    println!("Result 8: {:?}", value);

    // Iterators from other collections
    // [Type] -> [Iterator] -> [Type]
    let map = HashMap::from([("foo", 1), ("bar", 2), ("baz", 3)]);

    map.iter()
        .for_each(|(k, v)| println!("Result 9: {}, {}", k, v));
}
