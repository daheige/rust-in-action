// 引入pyo3包
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

// 在rust代码中调用python代码
fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?; // 导入python sys包
        let version: String = sys.getattr("version")?.extract()?; // 调用sys.version命令获取python版本

        let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py); // 导入os模块
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, Python version {}", user, version);
        Ok(())
    })
}
