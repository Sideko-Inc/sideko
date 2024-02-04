use camino::Utf8PathBuf;
use pyo3::{
    prelude::{pyfunction, pymodule, PyModule, Python},
    pyclass, wrap_pyfunction, PyResult,
};
use sideko::{cmds, utils};
use std::path::PathBuf;

#[pyclass]
#[derive(Clone, Debug)]
pub enum ProgrammingLanguage {
    Python,
    Ruby,
    Typescript,
    Rust,
    Go,
}

#[pyfunction]
pub fn generate_sdk(output: PathBuf, language: ProgrammingLanguage) {
    // let output_path = Utf8PathBuf::from_path_buf(output).expect("whoops");
    println!("{:?}", language);

    // Validate input
    // utils::validate_path(&output_path, &utils::PathKind::File, true).expect("validate path");

    // let _x = tokio::runtime::Runtime::new()
    //     .expect("runtime")
    //     .block_on(cmds::login::handle_login(&output_path));
}

#[pymodule]
pub fn sideko_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_sdk, m)?)?;
    m.add_class::<ProgrammingLanguage>()?;
    Ok(())
}
