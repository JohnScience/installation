
fn main() {
    println!("Python version based on info from CLI:\n\t{:?}", installation::python::get_version_from_cli().unwrap());
    println!("Python version based on info from PyO3:\n\t{:?}", installation::python::get_version_info_via_pyo3().unwrap());
}