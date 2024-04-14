use std::fmt;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
struct MyError {
    code: i32,
    descr: String,
}

impl MyError {
    fn new(code: i32, descr: String) -> Self {
        Self { code, descr }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "my_error: {} {}", self.code, self.descr)
    }
}


type MyResult<T> = std::result::Result<T, MyError>;

fn test() {
    Python::with_gil(|py| {
        let rust_test =
            PyModule::import_bound(py, "rust_test").expect("Unable to import rust_test module");
        let i2a = rust_test.getattr("i2a").expect("Error retrieving i2a attr");
        let concat = rust_test
            .getattr("concat")
            .expect("Error retrieving concat attr");
        let ret1: String = i2a
            .call1((13,))
            .expect("Error calling i2a with arg")
            .extract()
            .expect("Unable to convert output to string");
        println!("Found {}", ret1);
        let ret2 = concat
            .call1((10, 100))
            .expect("Error calling concat with args");
        let ret2 : String = generic_extract(ret2).expect("Not able to convert to str");
        println!("Found {}", ret2)
    })
}

fn generic_extract<'py, T>(result: Bound<'py, PyAny>) -> MyResult<T>
    where T:  FromPyObject<'py>, {
    match result.extract::<T>() {
        Ok(extracted) => Ok(extracted),
        Err(_) => Err(MyError::new(10, String::from("Wrong type extraction")))
    }
}

fn main() {
    println!("Let's try this !");
    test();
}
