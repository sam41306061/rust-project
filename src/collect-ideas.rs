use std::collections::HashSet;
use std::collections::HashMap;


fn collect_into_string () {
    let foo: String = vec!["this", "is", "a", "test"]
    .into_iter() 
    .collect();
println!("{:?}", foo);
}


fn collect_into_hash () {
  let foo: HashSet<isize> = vec![1,2,3]
  .into_iter()
  .collect();
println!("{:?}", foo);
}

fn collect_into_a_hash_map () {
    let foo: HashMap<&str, usize> = vec!["this","is","a","has","map"]
    .into_iter()
    .enumerate() // adds the index to the iterator!
                // one of the glories of rust is that we work with iterators
    .map(|(idx,item) | (item,idx)) // reverses order
    .collect();

    println!("{:?}", foo);
 }

fn main() {
    let data = vec![1,2,3]; // need this 
    let mut foo = data // to create this
    .iter() // create the iterator to go over the elements in the array
    .map(|x| x + 1); // do the plust one'ings
    
    let mut new_vector = vec![];

    while let Some(x) = foo.next(){
        new_vector.push(x)
    }

    
    println!("{:?}", new_vector);
    // function call
    collect_into_string();
    collect_into_hash();
    collect_into_a_hash_map();

}
