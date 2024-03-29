use std::collections::HashMap;
use woothee::parser::{Parser, WootheeResult};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
#[pyo3(name = "_woothee")]
fn fast_woothee(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfunction]
    pub fn parse(agent: &str) -> PyResult<HashMap<String, String>> {
        let parser = Parser::new();
        let result = parser.parse(agent);
        let r = match result {
            Some(r) => r,
            None => WootheeResult::new(),
        };
        let mut h = HashMap::new();
        h.insert("name".to_string(), r.name.to_owned());
        h.insert("category".to_string(), r.category.to_string());
        h.insert("os".to_string(), r.os.to_string());
        h.insert("os_version".to_string(), r.os_version.to_string());
        h.insert("browser_type".to_string(), r.browser_type.to_string());
        h.insert("version".to_string(), r.version.to_string());
        h.insert("vendor".to_string(), r.vendor.to_string());
        Ok(h)
    }

    #[pyfunction]
    pub fn is_crawler(agent: &str) -> PyResult<bool> {
        if agent.is_empty() || agent == "-" {
            return Ok(false);
        }

        let parser = Parser::new();
        let mut result = WootheeResult::new();
        Ok(parser.try_crawler(agent, &mut result))
    }

    m.add_wrapped(wrap_pyfunction!(parse))?;
    m.add_wrapped(wrap_pyfunction!(is_crawler))?;

    Ok(())
}

