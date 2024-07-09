use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::{pyfunction, pymodule, PyModule, Python},
    pyclass, wrap_pyfunction, PyResult,
};
use sideko::{cmds::generate, config, utils};
use sideko_api::schemas::GenerationLanguageEnum;
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
    fn to_gen_lang(&self) -> GenerationLanguageEnum {
        match self {
            Language::Python => GenerationLanguageEnum::Python,
            Language::Ruby => GenerationLanguageEnum::Ruby,
            Language::Typescript => GenerationLanguageEnum::Typescript,
            Language::Rust => GenerationLanguageEnum::Rust,
            Language::Go => GenerationLanguageEnum::Go,
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
    tests_mock_server_url: Option<String>,
) -> PyResult<()> {
    utils::init_logger(log::Level::Warn);
    config::load_config(config::config_bufs(vec![]));

    let params = generate::GenerateSdkParams {
        source: generate::OpenApiSource::from(&source),
        destination: dest,
        language: language.to_gen_lang(),
        base_url,
        package_name,
        tests_mock_server_url,
    };

    let cmd_res = tokio::runtime::Runtime::new()
        .expect("Failed starting blocking async runtime")
        .block_on(generate::handle_generate(&params));

    match cmd_res {
        Err(e) => Err(SidekoError::new_err(e.error_msg())),
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
