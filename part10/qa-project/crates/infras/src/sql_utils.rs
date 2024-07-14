// 根据len生成对应的(?,?,...)占位符，主要用于mysql where in (?,?)操作
pub fn gen_in_placeholder(len: usize) -> String {
    let parameters = (0..len)
        .into_iter()
        .map(|_| "?")
        .collect::<Vec<&str>>()
        .join(",");
    parameters
}
