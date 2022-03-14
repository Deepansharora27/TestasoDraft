#[macro_use]
pub use testaso::testaso_macro;

#[cfg(test)]
fn test_function() {
    testaso!( {
        struct mystruct: 8, 4096 => {
                   f1: 0,
                    f2: 8
                }
    })
}
