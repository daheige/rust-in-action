let str = '{"id":1,"name":"daheige","lang":"rust","is_married":true,"hobbies":["reading","music"],"address":{"city":"shenzhen","street":"guangming","post_code":"518000"}}';

// JSON字符串转换为JSON对象
let obj = JSON.parse(str);
console.log("obj:", obj);
console.log("obj.id:", obj.id, "name:", obj.name, "dev lang:", obj.lang);
// 使用for in遍历数组
for (key in obj.hobbies) {
    console.log("hobby is: ", obj.hobbies[key]);
}
