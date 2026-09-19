// build.rs
// 将提交记录编译进二进制文件中
// 构建之前需要设置环境变量
// 把 git commit、build time 编译进二进制
// export GIT_COMMIT=$(git rev-parse --short HEAD)
// export BUILD_TIME=$(date -u +%Y-%m-%dT%H:%M:%SZ)
// 这样在执行cargo build --release时，会把当前版本和构建时间打进二进制文件中
fn main() {
    let commit = std::env::var("GIT_COMMIT").unwrap_or_else(|_| "unknown".into());
    let time = std::env::var("BUILD_TIME").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=GIT_COMMIT={commit}");
    println!("cargo:rustc-env=BUILD_TIME={time}");
}
