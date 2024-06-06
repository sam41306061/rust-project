fn main() {
    let foo: Vec<_> = vec![1,2,3]
    .iter() // create the iterator to go over the elements in the array
    .map(|x| x + 1) // do the plust one'ings
    .collect(); // take the iterator and put it somewhere
    
    println!("{:?}", foo);
}
