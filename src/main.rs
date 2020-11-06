fn curry<F, T>(f: F, x: T) -> impl Fn(T) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    move |y| f(x, y)
}

fn main() {
    let add = |x, y| x + y;
    let closure = curry(add, 5);
    println!("closure(1) => {}", closure(1));
}
