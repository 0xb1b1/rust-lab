use pyo3::prelude::*;

#[pyfunction]
fn calc(n_terms: usize) -> PyResult<f64> {
    let numerator: f64 = 4.0;
    let mut denominator: f64 = 1.0;
    let mut operation: f64 = 1.0;
    let mut pi: f64 = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

#[pymodule]
fn picalc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc, m)?)?;
    Ok(())
}
