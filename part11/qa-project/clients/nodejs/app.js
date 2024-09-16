let services = require('./pb/qa_grpc_pb.js');
let messages = require('./pb/qa_pb.js');
let grpc = require('@grpc/grpc-js');

// 创建请求对象
let request = new messages.UserLoginRequest();
request.setUsername('daheige');
request.setPassword('123456');

// 创建grpc client
let client = new services.QAServiceClient(
    'localhost:50051',
    // '192.168.1.4:50051',
    grpc.credentials.createInsecure()
);

// 调用grpc微服务的方法
client.userLogin(request, function(err, data) {
    if (err) {
        console.error("user login error: ",err);
        return;
    }

    console.log(data);
    console.log("reply token:",data.getToken());
});
