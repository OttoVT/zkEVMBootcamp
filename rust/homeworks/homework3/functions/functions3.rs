// functions3.rs
// Make me compile! Execute `zustlings hint functions3` for hints :)

fn main() {
    call_this(3);
}

fn call_this(num: usize) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}
