let obj = {
    "id": 1,
    "name": "daheige",
    "lang": "rust",
    "is_married":true,
    "hobbies":["reading","music"],
    "address":{
        "city":"shenzhen",
        "street":"guangming",
        "post_code":518000
    }
};
let str = JSON.stringify(obj);
console.log("json str:",str);

