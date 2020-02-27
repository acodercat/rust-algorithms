use std::fmt::Debug;
trait PrintInOption {
    fn print_in_option(self);
}
// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`
// 或使用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为限定，因为那是要打印的内容。
    // 不这样做的话，很可能就用到错误的限定。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}
fn main() {
    let vec = vec![1, 2, 3];
    let a = 321;
    a.print_in_option();
    vec.print_in_option();
    let a = Box::new(12);
    let b = *a;
    format!("{:?}", 1);
}