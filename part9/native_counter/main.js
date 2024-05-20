const addon = require(".");
let txt = "a test to rust ffi for nodejs test this is test project";
let word = "test";
console.log("call count_words function");
let wc = addon.count_words(txt, word);
console.log(word + " count:", wc);

// 调用index.node中的hello函数
console.log("call hello function");
console.log(addon.hello());
console.log("恭喜你，使用rust为nodejs编写拓展成功！");
