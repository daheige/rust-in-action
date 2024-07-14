// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var qa_pb = require('./qa_pb.js');

function serialize_qa_AddAnswerReply(arg) {
  if (!(arg instanceof qa_pb.AddAnswerReply)) {
    throw new Error('Expected argument of type qa.AddAnswerReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AddAnswerReply(buffer_arg) {
  return qa_pb.AddAnswerReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AddAnswerRequest(arg) {
  if (!(arg instanceof qa_pb.AddAnswerRequest)) {
    throw new Error('Expected argument of type qa.AddAnswerRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AddAnswerRequest(buffer_arg) {
  return qa_pb.AddAnswerRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AddQuestionReply(arg) {
  if (!(arg instanceof qa_pb.AddQuestionReply)) {
    throw new Error('Expected argument of type qa.AddQuestionReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AddQuestionReply(buffer_arg) {
  return qa_pb.AddQuestionReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AddQuestionRequest(arg) {
  if (!(arg instanceof qa_pb.AddQuestionRequest)) {
    throw new Error('Expected argument of type qa.AddQuestionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AddQuestionRequest(buffer_arg) {
  return qa_pb.AddQuestionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerAgreeReply(arg) {
  if (!(arg instanceof qa_pb.AnswerAgreeReply)) {
    throw new Error('Expected argument of type qa.AnswerAgreeReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerAgreeReply(buffer_arg) {
  return qa_pb.AnswerAgreeReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerAgreeRequest(arg) {
  if (!(arg instanceof qa_pb.AnswerAgreeRequest)) {
    throw new Error('Expected argument of type qa.AnswerAgreeRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerAgreeRequest(buffer_arg) {
  return qa_pb.AnswerAgreeRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerDetailReply(arg) {
  if (!(arg instanceof qa_pb.AnswerDetailReply)) {
    throw new Error('Expected argument of type qa.AnswerDetailReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerDetailReply(buffer_arg) {
  return qa_pb.AnswerDetailReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerDetailRequest(arg) {
  if (!(arg instanceof qa_pb.AnswerDetailRequest)) {
    throw new Error('Expected argument of type qa.AnswerDetailRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerDetailRequest(buffer_arg) {
  return qa_pb.AnswerDetailRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerListReply(arg) {
  if (!(arg instanceof qa_pb.AnswerListReply)) {
    throw new Error('Expected argument of type qa.AnswerListReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerListReply(buffer_arg) {
  return qa_pb.AnswerListReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_AnswerListRequest(arg) {
  if (!(arg instanceof qa_pb.AnswerListRequest)) {
    throw new Error('Expected argument of type qa.AnswerListRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_AnswerListRequest(buffer_arg) {
  return qa_pb.AnswerListRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_DeleteAnswerReply(arg) {
  if (!(arg instanceof qa_pb.DeleteAnswerReply)) {
    throw new Error('Expected argument of type qa.DeleteAnswerReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_DeleteAnswerReply(buffer_arg) {
  return qa_pb.DeleteAnswerReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_DeleteAnswerRequest(arg) {
  if (!(arg instanceof qa_pb.DeleteAnswerRequest)) {
    throw new Error('Expected argument of type qa.DeleteAnswerRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_DeleteAnswerRequest(buffer_arg) {
  return qa_pb.DeleteAnswerRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_DeleteQuestionReply(arg) {
  if (!(arg instanceof qa_pb.DeleteQuestionReply)) {
    throw new Error('Expected argument of type qa.DeleteQuestionReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_DeleteQuestionReply(buffer_arg) {
  return qa_pb.DeleteQuestionReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_DeleteQuestionRequest(arg) {
  if (!(arg instanceof qa_pb.DeleteQuestionRequest)) {
    throw new Error('Expected argument of type qa.DeleteQuestionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_DeleteQuestionRequest(buffer_arg) {
  return qa_pb.DeleteQuestionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_LatestQuestionsReply(arg) {
  if (!(arg instanceof qa_pb.LatestQuestionsReply)) {
    throw new Error('Expected argument of type qa.LatestQuestionsReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_LatestQuestionsReply(buffer_arg) {
  return qa_pb.LatestQuestionsReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_LatestQuestionsRequest(arg) {
  if (!(arg instanceof qa_pb.LatestQuestionsRequest)) {
    throw new Error('Expected argument of type qa.LatestQuestionsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_LatestQuestionsRequest(buffer_arg) {
  return qa_pb.LatestQuestionsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_QuestionDetailReply(arg) {
  if (!(arg instanceof qa_pb.QuestionDetailReply)) {
    throw new Error('Expected argument of type qa.QuestionDetailReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_QuestionDetailReply(buffer_arg) {
  return qa_pb.QuestionDetailReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_QuestionDetailRequest(arg) {
  if (!(arg instanceof qa_pb.QuestionDetailRequest)) {
    throw new Error('Expected argument of type qa.QuestionDetailRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_QuestionDetailRequest(buffer_arg) {
  return qa_pb.QuestionDetailRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UpdateAnswerReply(arg) {
  if (!(arg instanceof qa_pb.UpdateAnswerReply)) {
    throw new Error('Expected argument of type qa.UpdateAnswerReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UpdateAnswerReply(buffer_arg) {
  return qa_pb.UpdateAnswerReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UpdateAnswerRequest(arg) {
  if (!(arg instanceof qa_pb.UpdateAnswerRequest)) {
    throw new Error('Expected argument of type qa.UpdateAnswerRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UpdateAnswerRequest(buffer_arg) {
  return qa_pb.UpdateAnswerRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UpdateQuestionReply(arg) {
  if (!(arg instanceof qa_pb.UpdateQuestionReply)) {
    throw new Error('Expected argument of type qa.UpdateQuestionReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UpdateQuestionReply(buffer_arg) {
  return qa_pb.UpdateQuestionReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UpdateQuestionRequest(arg) {
  if (!(arg instanceof qa_pb.UpdateQuestionRequest)) {
    throw new Error('Expected argument of type qa.UpdateQuestionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UpdateQuestionRequest(buffer_arg) {
  return qa_pb.UpdateQuestionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserLoginReply(arg) {
  if (!(arg instanceof qa_pb.UserLoginReply)) {
    throw new Error('Expected argument of type qa.UserLoginReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserLoginReply(buffer_arg) {
  return qa_pb.UserLoginReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserLoginRequest(arg) {
  if (!(arg instanceof qa_pb.UserLoginRequest)) {
    throw new Error('Expected argument of type qa.UserLoginRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserLoginRequest(buffer_arg) {
  return qa_pb.UserLoginRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserLogoutReply(arg) {
  if (!(arg instanceof qa_pb.UserLogoutReply)) {
    throw new Error('Expected argument of type qa.UserLogoutReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserLogoutReply(buffer_arg) {
  return qa_pb.UserLogoutReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserLogoutRequest(arg) {
  if (!(arg instanceof qa_pb.UserLogoutRequest)) {
    throw new Error('Expected argument of type qa.UserLogoutRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserLogoutRequest(buffer_arg) {
  return qa_pb.UserLogoutRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserRegisterReply(arg) {
  if (!(arg instanceof qa_pb.UserRegisterReply)) {
    throw new Error('Expected argument of type qa.UserRegisterReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserRegisterReply(buffer_arg) {
  return qa_pb.UserRegisterReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_UserRegisterRequest(arg) {
  if (!(arg instanceof qa_pb.UserRegisterRequest)) {
    throw new Error('Expected argument of type qa.UserRegisterRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_UserRegisterRequest(buffer_arg) {
  return qa_pb.UserRegisterRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_VerifyTokenReply(arg) {
  if (!(arg instanceof qa_pb.VerifyTokenReply)) {
    throw new Error('Expected argument of type qa.VerifyTokenReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_VerifyTokenReply(buffer_arg) {
  return qa_pb.VerifyTokenReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_qa_VerifyTokenRequest(arg) {
  if (!(arg instanceof qa_pb.VerifyTokenRequest)) {
    throw new Error('Expected argument of type qa.VerifyTokenRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_qa_VerifyTokenRequest(buffer_arg) {
  return qa_pb.VerifyTokenRequest.deserializeBinary(new Uint8Array(buffer_arg));
}


// qa服务接口定义
var QAServiceService = exports.QAServiceService = {
  // 用户登录
userLogin: {
    path: '/qa.QAService/UserLogin',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.UserLoginRequest,
    responseType: qa_pb.UserLoginReply,
    requestSerialize: serialize_qa_UserLoginRequest,
    requestDeserialize: deserialize_qa_UserLoginRequest,
    responseSerialize: serialize_qa_UserLoginReply,
    responseDeserialize: deserialize_qa_UserLoginReply,
  },
  // 用户退出
userLogout: {
    path: '/qa.QAService/UserLogout',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.UserLogoutRequest,
    responseType: qa_pb.UserLogoutReply,
    requestSerialize: serialize_qa_UserLogoutRequest,
    requestDeserialize: deserialize_qa_UserLogoutRequest,
    responseSerialize: serialize_qa_UserLogoutReply,
    responseDeserialize: deserialize_qa_UserLogoutReply,
  },
  // 用户注册
userRegister: {
    path: '/qa.QAService/UserRegister',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.UserRegisterRequest,
    responseType: qa_pb.UserRegisterReply,
    requestSerialize: serialize_qa_UserRegisterRequest,
    requestDeserialize: deserialize_qa_UserRegisterRequest,
    responseSerialize: serialize_qa_UserRegisterReply,
    responseDeserialize: deserialize_qa_UserRegisterReply,
  },
  // 验证登录的token是否有效
verifyToken: {
    path: '/qa.QAService/VerifyToken',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.VerifyTokenRequest,
    responseType: qa_pb.VerifyTokenReply,
    requestSerialize: serialize_qa_VerifyTokenRequest,
    requestDeserialize: deserialize_qa_VerifyTokenRequest,
    responseSerialize: serialize_qa_VerifyTokenReply,
    responseDeserialize: deserialize_qa_VerifyTokenReply,
  },
  // 发表问题
addQuestion: {
    path: '/qa.QAService/AddQuestion',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.AddQuestionRequest,
    responseType: qa_pb.AddQuestionReply,
    requestSerialize: serialize_qa_AddQuestionRequest,
    requestDeserialize: deserialize_qa_AddQuestionRequest,
    responseSerialize: serialize_qa_AddQuestionReply,
    responseDeserialize: deserialize_qa_AddQuestionReply,
  },
  // 删除问题
deleteQuestion: {
    path: '/qa.QAService/DeleteQuestion',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.DeleteQuestionRequest,
    responseType: qa_pb.DeleteQuestionReply,
    requestSerialize: serialize_qa_DeleteQuestionRequest,
    requestDeserialize: deserialize_qa_DeleteQuestionRequest,
    responseSerialize: serialize_qa_DeleteQuestionReply,
    responseDeserialize: deserialize_qa_DeleteQuestionReply,
  },
  // 修改问题
updateQuestion: {
    path: '/qa.QAService/UpdateQuestion',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.UpdateQuestionRequest,
    responseType: qa_pb.UpdateQuestionReply,
    requestSerialize: serialize_qa_UpdateQuestionRequest,
    requestDeserialize: deserialize_qa_UpdateQuestionRequest,
    responseSerialize: serialize_qa_UpdateQuestionReply,
    responseDeserialize: deserialize_qa_UpdateQuestionReply,
  },
  // 查看问题详情
questionDetail: {
    path: '/qa.QAService/QuestionDetail',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.QuestionDetailRequest,
    responseType: qa_pb.QuestionDetailReply,
    requestSerialize: serialize_qa_QuestionDetailRequest,
    requestDeserialize: deserialize_qa_QuestionDetailRequest,
    responseSerialize: serialize_qa_QuestionDetailReply,
    responseDeserialize: deserialize_qa_QuestionDetailReply,
  },
  // 最新问题列表（采用下拉分页形式获取数据，按照id desc倒序）
latestQuestions: {
    path: '/qa.QAService/LatestQuestions',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.LatestQuestionsRequest,
    responseType: qa_pb.LatestQuestionsReply,
    requestSerialize: serialize_qa_LatestQuestionsRequest,
    requestDeserialize: deserialize_qa_LatestQuestionsRequest,
    responseSerialize: serialize_qa_LatestQuestionsReply,
    responseDeserialize: deserialize_qa_LatestQuestionsReply,
  },
  // 回答列表
answerList: {
    path: '/qa.QAService/AnswerList',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.AnswerListRequest,
    responseType: qa_pb.AnswerListReply,
    requestSerialize: serialize_qa_AnswerListRequest,
    requestDeserialize: deserialize_qa_AnswerListRequest,
    responseSerialize: serialize_qa_AnswerListReply,
    responseDeserialize: deserialize_qa_AnswerListReply,
  },
  // 添加问题回答
addAnswer: {
    path: '/qa.QAService/AddAnswer',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.AddAnswerRequest,
    responseType: qa_pb.AddAnswerReply,
    requestSerialize: serialize_qa_AddAnswerRequest,
    requestDeserialize: deserialize_qa_AddAnswerRequest,
    responseSerialize: serialize_qa_AddAnswerReply,
    responseDeserialize: deserialize_qa_AddAnswerReply,
  },
  // 删除问题对应的回答
deleteAnswer: {
    path: '/qa.QAService/DeleteAnswer',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.DeleteAnswerRequest,
    responseType: qa_pb.DeleteAnswerReply,
    requestSerialize: serialize_qa_DeleteAnswerRequest,
    requestDeserialize: deserialize_qa_DeleteAnswerRequest,
    responseSerialize: serialize_qa_DeleteAnswerReply,
    responseDeserialize: deserialize_qa_DeleteAnswerReply,
  },
  // 修改回答
updateAnswer: {
    path: '/qa.QAService/UpdateAnswer',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.UpdateAnswerRequest,
    responseType: qa_pb.UpdateAnswerReply,
    requestSerialize: serialize_qa_UpdateAnswerRequest,
    requestDeserialize: deserialize_qa_UpdateAnswerRequest,
    responseSerialize: serialize_qa_UpdateAnswerReply,
    responseDeserialize: deserialize_qa_UpdateAnswerReply,
  },
  // 查看答案详情
answerDetail: {
    path: '/qa.QAService/AnswerDetail',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.AnswerDetailRequest,
    responseType: qa_pb.AnswerDetailReply,
    requestSerialize: serialize_qa_AnswerDetailRequest,
    requestDeserialize: deserialize_qa_AnswerDetailRequest,
    responseSerialize: serialize_qa_AnswerDetailReply,
    responseDeserialize: deserialize_qa_AnswerDetailReply,
  },
  // 用户点赞回答和取消点赞
answerAgree: {
    path: '/qa.QAService/AnswerAgree',
    requestStream: false,
    responseStream: false,
    requestType: qa_pb.AnswerAgreeRequest,
    responseType: qa_pb.AnswerAgreeReply,
    requestSerialize: serialize_qa_AnswerAgreeRequest,
    requestDeserialize: deserialize_qa_AnswerAgreeRequest,
    responseSerialize: serialize_qa_AnswerAgreeReply,
    responseDeserialize: deserialize_qa_AnswerAgreeReply,
  },
};

exports.QAServiceClient = grpc.makeGenericClientConstructor(QAServiceService);
