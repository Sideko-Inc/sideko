pub fn foo() {
    print!("foo bar")
}

// use camino::Utf8PathBuf;
// use pyo3::{
//     prelude::{pyfunction, pymodule, PyModule, Python},
//     wrap_pyfunction, PyResult,
// };
// use std::str::FromStr;

// pub mod cmds;
// pub mod result;
// pub mod styles;
// pub mod utils;

// #[pyfunction]
// pub fn run_login() {
//     let output_path = {
//         let home = std::env::var("HOME").expect("whoops");
//         let mut utf_buf = Utf8PathBuf::from_str(&home).expect("whoops 2");
//         utf_buf.push(".sideko");
//         utf_buf
//     };

//     // Validate input
//     utils::validate_path(&output_path, &utils::PathKind::File, true).expect("validate path");

//     let _x = tokio::runtime::Runtime::new()
//         .expect("runtime")
//         .block_on(cmds::login::handle_login(&output_path));
// }

// #[pymodule]
// pub fn sideko(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(run_login, m)?)?;
//     Ok(())
// }
