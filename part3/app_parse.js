let str = '{"id":1,"name":"daheige","lang":"rust","is_married":true,"hobbies":["reading","music"],"address":{"city":"shenzhen","street":"guangming","post_code":518000}}';
let obj = JSON.parse(str);
console.log("obj:",obj);
console.log("obj.id: ",obj.id,"name: ",obj.name);