use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::{pyfunction, pymodule, PyModule, Python},
    wrap_pyfunction, PyResult,
};

create_exception!(sideko_py, SidekoError, PyException);

#[pyfunction]
pub fn cli(py: Python) -> PyResult<()> {
    // https://www.maturin.rs/bindings.html?highlight=scripts#both-binary-and-library
    let args = py
        .import("sys")?
        .getattr("argv")?
        .extract::<Vec<String>>()?;

    let res = tokio::runtime::Runtime::new()
        .expect("Failed starting blocking async runtime")
        .block_on(sideko::cli::cli(args));

    if res.is_err() {
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}

#[pymodule]
pub fn sideko_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cli, m)?)?;

    Ok(())
}
