// 引入pyo3包
use pyo3::prelude::*;

// sum_as_string 用于将两个i64的数字相加并转换为字符串格式
#[pyfunction]
fn sum_as_string(a: i64, b: i64) -> String {
    (a + b).to_string()
}

// explode用于将字符串按照指定的分割符sep进行分割，返回一个字符串列表
#[pyfunction]
fn explode<'a>(s: &'a str, sep: &'a str) -> Vec<&'a str> {
    let v = s.split(sep).collect();
    v
}

// implode用于将字符串列表按照连接符sep进行连接，返回一个字符串
#[pyfunction]
fn implode(v: Vec<String>, sep: &str) -> String {
    let s = v.join(sep);
    s
}

// #[pymodule] 用于声明python包名字为string_utils
#[pymodule]
fn string_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 通过add_function将这些函数注册到模块string_utils中
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(explode, m)?)?;
    m.add_function(wrap_pyfunction!(implode, m)?)?;
    Ok(())
}
