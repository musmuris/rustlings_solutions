// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn call_me() 
{

}

fn main() {
    call_me();
    let s = "hello";
    let s2 = s.to_string();
    let v = vec![1,2,3];
    let a = [1,2,3,4];
    let ar = &a[2..3];

    print_type_of(&s);
    print_type_of(&s2);
    print_type_of(&v);
    print_type_of(&a);
    print_type_of(&ar);
}
