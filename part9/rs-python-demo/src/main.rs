use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::ffi::c_str;

// 在rust代码中调用python代码
fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;

        // 调用sys.version命令获取python版本
        let version: String = sys.getattr("version")?.extract()?;

        // 导入os模块，获取用户名
        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}