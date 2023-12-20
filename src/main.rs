//use cpython::{Python, PyResult, PyObject};
use pyo3::prelude::*;

/*
fn test1() {
   // Initialize the Python interpreter
   let gil = Python::acquire_gil();
   let py = gil.python();

   let python_module = py.import("rust_test").expect("Error importing rust_test");

   let result: PyResult<PyObject> = python_module.call(py, "i2a", (42,), None);
   match result {
       Ok(value) => {
           // Do something with the result
           let py_str: PyResult<String> = value.extract().expect("Error extracting result");
           println!("Result from Python: {}", py_str);
       }
       Err(err) => {
           // Handle the error
           eprintln!("Error calling i2a function: {:?}", err);
       }
   }
}
*/

fn test2() {
    Python::with_gil(|py| {
        let rust_test =
            PyModule::import(py, "rust_test").expect("Unable to import rust_test module");
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
        let ret2: String = concat
            .call1((10, 100))
            .expect("Error calling concat with args")
            .extract()
            .expect("Unable to convert output to string");

        println!("Found {}", ret2)
    })
}
fn main() {
    println!("Let's try this !");
    test2();
}
