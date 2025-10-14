// 调用index.node中的count_words函数
const addon = require(".");
let txt = "a test to rust ffi for nodejs test this is test project";
let word = "test";
console.log("call count_words function");
let wc = addon.count_words(txt, word);
console.log(word + " count:", wc);
console.log("恭喜你，使用rust为nodejs编写拓展成功！");

// 调用hello函数
console.log(addon.hello());
