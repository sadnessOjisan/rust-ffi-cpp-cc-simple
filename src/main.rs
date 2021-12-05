#[link(name = "foo", kind = "static")]
extern "C" {
    fn foo_function();
    fn bar_function(x: i32) -> i32;
}

pub fn call() {
    unsafe {
        foo_function();
        // bar_function(42);
    }
}

fn main() {
    call()
}
