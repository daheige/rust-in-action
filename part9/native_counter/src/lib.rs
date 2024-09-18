// 引入neon包里的相关模块
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn count_words(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // count_words函数在nodejs环境下执行的第一个参数
    let txt = cx.argument::<JsString>(0)?.value(&mut cx);
    // count_words函数在nodejs环境下执行的第二个参数
    let word = cx.argument::<JsString>(1)?.value(&mut cx);

    // 按照空格进行分割的结果是一个字符串数组，再通过filter过滤比较字符串，统计word出现的次数。
    // 这里需要将结果强制转换为f64格式，因为js number都是float64类型。
    Ok(cx.number(txt.split(" ").filter(|s| s == &word).count() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("count_words", count_words)?;
    Ok(())
}
