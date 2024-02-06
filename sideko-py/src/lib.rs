use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::{pyfunction, pymodule, PyModule, Python},
    pyclass, wrap_pyfunction, PyResult,
};
use sideko::{cmds::generate, config, utils};
use std::path::PathBuf;

#[pyclass]
#[derive(Clone, Debug)]
pub enum Language {
    Python,
    Ruby,
    Typescript,
    Rust,
    Go,
}

impl Language {
    fn to_sideko_programming_lang(&self) -> generate::ProgrammingLanguage {
        match self {
            Language::Python => generate::ProgrammingLanguage::Python,
            Language::Ruby => generate::ProgrammingLanguage::Ruby,
            Language::Typescript => generate::ProgrammingLanguage::Typescript,
            Language::Rust => generate::ProgrammingLanguage::Rust,
            Language::Go => generate::ProgrammingLanguage::Go,
        }
    }
}

create_exception!(sideko_py, SidekoError, PyException);

#[pyfunction]
pub fn generate_sdk(
    language: Language,
    source: String,
    dest: PathBuf,
    base_url: Option<String>,
    package_name: Option<String>,
) -> PyResult<()> {
    utils::init_logger(log::Level::Warn);
    config::load_config(config::config_bufs(vec![]));

    let params = generate::GenerateSdkParams {
        source: generate::OpenApiSource::from(&source),
        destination: dest,
        language: language.to_sideko_programming_lang(),
        base_url,
        package_name,
    };

    let cmd_res = tokio::runtime::Runtime::new()
        .expect("Failed starting blocking async runtime")
        .block_on(generate::handle_generate(&params));

    match cmd_res {
        Err(
            sideko::result::Error::ArgumentError(msg)
            | sideko::result::Error::General(msg)
            | sideko::result::Error::ReqwestError(msg, ..)
            | sideko::result::Error::ResponseError(msg, ..)
            | sideko::result::Error::IoError(msg, ..),
        ) => Err(SidekoError::new_err(msg)),
        Ok(_) => Ok(()),
    }
}

#[pyfunction]
pub fn cli(py: Python) -> PyResult<()> {
    // https://www.maturin.rs/bindings.html?highlight=scripts#both-binary-and-library
    let args = py
        .import("sys")?
        .getattr("argv")?
        .extract::<Vec<String>>()?;

    let _ = tokio::runtime::Runtime::new()
        .expect("Failed starting blocking async runtime")
        .block_on(sideko::cli::cli(args));

    Ok(())
}

#[pymodule]
pub fn sideko_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_sdk, m)?)?;
    m.add_function(wrap_pyfunction!(cli, m)?)?;
    m.add_class::<Language>()?;
    m.add("SidekoError", py.get_type::<SidekoError>())?;
    Ok(())
}
