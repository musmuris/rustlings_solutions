// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

fn main() {
    let mut vec0 = Vec::new();

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    fill_vec_mut(&mut vec0);
    vec0.push(88);
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_mut(vec: &mut Vec<i32>) {    
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
