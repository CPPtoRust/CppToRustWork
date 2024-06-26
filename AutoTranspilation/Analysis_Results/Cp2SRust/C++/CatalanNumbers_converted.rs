// Using Namespace directives are not yet supported in this transpiler... Copying as it is
// using namespace std ;
fn catalan(mut n: u32) -> i32 {
    if n <= 1 {
        return 1;
    }
    let mut res: i32 = 0;
    let mut i: i32 = 0;
    while i < n {
        res += catalan(i) * catalan(n - i - 1);
        i += 1;
    }
    return res;
}
fn main() {
    let mut i: i32 = 0;
    while i < 10 {
        println!("{} ", catalan(i));
        i += 1;
    }
    return 0;
}
