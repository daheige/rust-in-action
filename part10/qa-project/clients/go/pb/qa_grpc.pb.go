// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v4.25.1
// source: qa.proto

package pb

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	QAService_UserLogin_FullMethodName         = "/qa.QAService/UserLogin"
	QAService_UserLogout_FullMethodName        = "/qa.QAService/UserLogout"
	QAService_UserRegister_FullMethodName      = "/qa.QAService/UserRegister"
	QAService_AddQuestion_FullMethodName       = "/qa.QAService/AddQuestion"
	QAService_DeleteQuestion_FullMethodName    = "/qa.QAService/DeleteQuestion"
	QAService_UpdateQuestion_FullMethodName    = "/qa.QAService/UpdateQuestion"
	QAService_QuestionDetail_FullMethodName    = "/qa.QAService/QuestionDetail"
	QAService_LatestQuestions_FullMethodName   = "/qa.QAService/LatestQuestions"
	QAService_AnswerList_FullMethodName        = "/qa.QAService/AnswerList"
	QAService_AddAnswer_FullMethodName         = "/qa.QAService/AddAnswer"
	QAService_DeleteAnswer_FullMethodName      = "/qa.QAService/DeleteAnswer"
	QAService_UpdateAnswer_FullMethodName      = "/qa.QAService/UpdateAnswer"
	QAService_AnswerAgree_FullMethodName       = "/qa.QAService/AnswerAgree"
	QAService_QuestionReadCount_FullMethodName = "/qa.QAService/QuestionReadCount"
)

// QAServiceClient is the client API for QAService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type QAServiceClient interface {
	// 用户登录
	UserLogin(ctx context.Context, in *UserLoginRequest, opts ...grpc.CallOption) (*UserLoginReply, error)
	// 用户退出
	UserLogout(ctx context.Context, in *UserLogoutRequest, opts ...grpc.CallOption) (*UserLogoutReply, error)
	// 用户注册
	UserRegister(ctx context.Context, in *UserRegisterRequest, opts ...grpc.CallOption) (*UserRegisterReply, error)
	// 发表问题
	AddQuestion(ctx context.Context, in *AddQuestionRequest, opts ...grpc.CallOption) (*AddQuestionReply, error)
	// 删除问题
	DeleteQuestion(ctx context.Context, in *DeleteQuestionRequest, opts ...grpc.CallOption) (*DeleteQuestionReply, error)
	// 修改问题
	UpdateQuestion(ctx context.Context, in *UpdateQuestionRequest, opts ...grpc.CallOption) (*UpdateQuestionReply, error)
	// 查看问题详情
	QuestionDetail(ctx context.Context, in *QuestionDetailRequest, opts ...grpc.CallOption) (*QuestionDetailReply, error)
	// 最新问题列表（采用下拉分页形式获取数据，按照id desc倒序）
	LatestQuestions(ctx context.Context, in *LatestQuestionsRequest, opts ...grpc.CallOption) (*LatestQuestionsReply, error)
	// 回答列表
	AnswerList(ctx context.Context, in *AnswerListRequest, opts ...grpc.CallOption) (*AnswerListReply, error)
	// 添加问题回答
	AddAnswer(ctx context.Context, in *AddAnswerRequest, opts ...grpc.CallOption) (*AddAnswerReply, error)
	// 删除问题对应的回答
	DeleteAnswer(ctx context.Context, in *DeleteAnswerRequest, opts ...grpc.CallOption) (*DeleteAnswerReply, error)
	// 修改回答
	UpdateAnswer(ctx context.Context, in *UpdateAnswerRequest, opts ...grpc.CallOption) (*UpdateAnswerReply, error)
	// 用户点赞回答
	AnswerAgree(ctx context.Context, in *AnswerAgreeRequest, opts ...grpc.CallOption) (*AnswerAgreeReply, error)
	// 问题阅读数
	QuestionReadCount(ctx context.Context, in *QuestionReadCountRequest, opts ...grpc.CallOption) (*QuestionReadCountReply, error)
}

type qAServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewQAServiceClient(cc grpc.ClientConnInterface) QAServiceClient {
	return &qAServiceClient{cc}
}

func (c *qAServiceClient) UserLogin(ctx context.Context, in *UserLoginRequest, opts ...grpc.CallOption) (*UserLoginReply, error) {
	out := new(UserLoginReply)
	err := c.cc.Invoke(ctx, QAService_UserLogin_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) UserLogout(ctx context.Context, in *UserLogoutRequest, opts ...grpc.CallOption) (*UserLogoutReply, error) {
	out := new(UserLogoutReply)
	err := c.cc.Invoke(ctx, QAService_UserLogout_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) UserRegister(ctx context.Context, in *UserRegisterRequest, opts ...grpc.CallOption) (*UserRegisterReply, error) {
	out := new(UserRegisterReply)
	err := c.cc.Invoke(ctx, QAService_UserRegister_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) AddQuestion(ctx context.Context, in *AddQuestionRequest, opts ...grpc.CallOption) (*AddQuestionReply, error) {
	out := new(AddQuestionReply)
	err := c.cc.Invoke(ctx, QAService_AddQuestion_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) DeleteQuestion(ctx context.Context, in *DeleteQuestionRequest, opts ...grpc.CallOption) (*DeleteQuestionReply, error) {
	out := new(DeleteQuestionReply)
	err := c.cc.Invoke(ctx, QAService_DeleteQuestion_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) UpdateQuestion(ctx context.Context, in *UpdateQuestionRequest, opts ...grpc.CallOption) (*UpdateQuestionReply, error) {
	out := new(UpdateQuestionReply)
	err := c.cc.Invoke(ctx, QAService_UpdateQuestion_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) QuestionDetail(ctx context.Context, in *QuestionDetailRequest, opts ...grpc.CallOption) (*QuestionDetailReply, error) {
	out := new(QuestionDetailReply)
	err := c.cc.Invoke(ctx, QAService_QuestionDetail_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) LatestQuestions(ctx context.Context, in *LatestQuestionsRequest, opts ...grpc.CallOption) (*LatestQuestionsReply, error) {
	out := new(LatestQuestionsReply)
	err := c.cc.Invoke(ctx, QAService_LatestQuestions_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) AnswerList(ctx context.Context, in *AnswerListRequest, opts ...grpc.CallOption) (*AnswerListReply, error) {
	out := new(AnswerListReply)
	err := c.cc.Invoke(ctx, QAService_AnswerList_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) AddAnswer(ctx context.Context, in *AddAnswerRequest, opts ...grpc.CallOption) (*AddAnswerReply, error) {
	out := new(AddAnswerReply)
	err := c.cc.Invoke(ctx, QAService_AddAnswer_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) DeleteAnswer(ctx context.Context, in *DeleteAnswerRequest, opts ...grpc.CallOption) (*DeleteAnswerReply, error) {
	out := new(DeleteAnswerReply)
	err := c.cc.Invoke(ctx, QAService_DeleteAnswer_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) UpdateAnswer(ctx context.Context, in *UpdateAnswerRequest, opts ...grpc.CallOption) (*UpdateAnswerReply, error) {
	out := new(UpdateAnswerReply)
	err := c.cc.Invoke(ctx, QAService_UpdateAnswer_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) AnswerAgree(ctx context.Context, in *AnswerAgreeRequest, opts ...grpc.CallOption) (*AnswerAgreeReply, error) {
	out := new(AnswerAgreeReply)
	err := c.cc.Invoke(ctx, QAService_AnswerAgree_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *qAServiceClient) QuestionReadCount(ctx context.Context, in *QuestionReadCountRequest, opts ...grpc.CallOption) (*QuestionReadCountReply, error) {
	out := new(QuestionReadCountReply)
	err := c.cc.Invoke(ctx, QAService_QuestionReadCount_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// QAServiceServer is the server API for QAService service.
// All implementations must embed UnimplementedQAServiceServer
// for forward compatibility
type QAServiceServer interface {
	// 用户登录
	UserLogin(context.Context, *UserLoginRequest) (*UserLoginReply, error)
	// 用户退出
	UserLogout(context.Context, *UserLogoutRequest) (*UserLogoutReply, error)
	// 用户注册
	UserRegister(context.Context, *UserRegisterRequest) (*UserRegisterReply, error)
	// 发表问题
	AddQuestion(context.Context, *AddQuestionRequest) (*AddQuestionReply, error)
	// 删除问题
	DeleteQuestion(context.Context, *DeleteQuestionRequest) (*DeleteQuestionReply, error)
	// 修改问题
	UpdateQuestion(context.Context, *UpdateQuestionRequest) (*UpdateQuestionReply, error)
	// 查看问题详情
	QuestionDetail(context.Context, *QuestionDetailRequest) (*QuestionDetailReply, error)
	// 最新问题列表（采用下拉分页形式获取数据，按照id desc倒序）
	LatestQuestions(context.Context, *LatestQuestionsRequest) (*LatestQuestionsReply, error)
	// 回答列表
	AnswerList(context.Context, *AnswerListRequest) (*AnswerListReply, error)
	// 添加问题回答
	AddAnswer(context.Context, *AddAnswerRequest) (*AddAnswerReply, error)
	// 删除问题对应的回答
	DeleteAnswer(context.Context, *DeleteAnswerRequest) (*DeleteAnswerReply, error)
	// 修改回答
	UpdateAnswer(context.Context, *UpdateAnswerRequest) (*UpdateAnswerReply, error)
	// 用户点赞回答
	AnswerAgree(context.Context, *AnswerAgreeRequest) (*AnswerAgreeReply, error)
	// 问题阅读数
	QuestionReadCount(context.Context, *QuestionReadCountRequest) (*QuestionReadCountReply, error)
	mustEmbedUnimplementedQAServiceServer()
}

// UnimplementedQAServiceServer must be embedded to have forward compatible implementations.
type UnimplementedQAServiceServer struct {
}

func (UnimplementedQAServiceServer) UserLogin(context.Context, *UserLoginRequest) (*UserLoginReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UserLogin not implemented")
}
func (UnimplementedQAServiceServer) UserLogout(context.Context, *UserLogoutRequest) (*UserLogoutReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UserLogout not implemented")
}
func (UnimplementedQAServiceServer) UserRegister(context.Context, *UserRegisterRequest) (*UserRegisterReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UserRegister not implemented")
}
func (UnimplementedQAServiceServer) AddQuestion(context.Context, *AddQuestionRequest) (*AddQuestionReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddQuestion not implemented")
}
func (UnimplementedQAServiceServer) DeleteQuestion(context.Context, *DeleteQuestionRequest) (*DeleteQuestionReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteQuestion not implemented")
}
func (UnimplementedQAServiceServer) UpdateQuestion(context.Context, *UpdateQuestionRequest) (*UpdateQuestionReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateQuestion not implemented")
}
func (UnimplementedQAServiceServer) QuestionDetail(context.Context, *QuestionDetailRequest) (*QuestionDetailReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method QuestionDetail not implemented")
}
func (UnimplementedQAServiceServer) LatestQuestions(context.Context, *LatestQuestionsRequest) (*LatestQuestionsReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method LatestQuestions not implemented")
}
func (UnimplementedQAServiceServer) AnswerList(context.Context, *AnswerListRequest) (*AnswerListReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AnswerList not implemented")
}
func (UnimplementedQAServiceServer) AddAnswer(context.Context, *AddAnswerRequest) (*AddAnswerReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddAnswer not implemented")
}
func (UnimplementedQAServiceServer) DeleteAnswer(context.Context, *DeleteAnswerRequest) (*DeleteAnswerReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteAnswer not implemented")
}
func (UnimplementedQAServiceServer) UpdateAnswer(context.Context, *UpdateAnswerRequest) (*UpdateAnswerReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateAnswer not implemented")
}
func (UnimplementedQAServiceServer) AnswerAgree(context.Context, *AnswerAgreeRequest) (*AnswerAgreeReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AnswerAgree not implemented")
}
func (UnimplementedQAServiceServer) QuestionReadCount(context.Context, *QuestionReadCountRequest) (*QuestionReadCountReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method QuestionReadCount not implemented")
}
func (UnimplementedQAServiceServer) mustEmbedUnimplementedQAServiceServer() {}

// UnsafeQAServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to QAServiceServer will
// result in compilation errors.
type UnsafeQAServiceServer interface {
	mustEmbedUnimplementedQAServiceServer()
}

func RegisterQAServiceServer(s grpc.ServiceRegistrar, srv QAServiceServer) {
	s.RegisterService(&QAService_ServiceDesc, srv)
}

func _QAService_UserLogin_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserLoginRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).UserLogin(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_UserLogin_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).UserLogin(ctx, req.(*UserLoginRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_UserLogout_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserLogoutRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).UserLogout(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_UserLogout_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).UserLogout(ctx, req.(*UserLogoutRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_UserRegister_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserRegisterRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).UserRegister(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_UserRegister_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).UserRegister(ctx, req.(*UserRegisterRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_AddQuestion_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AddQuestionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).AddQuestion(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_AddQuestion_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).AddQuestion(ctx, req.(*AddQuestionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_DeleteQuestion_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteQuestionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).DeleteQuestion(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_DeleteQuestion_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).DeleteQuestion(ctx, req.(*DeleteQuestionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_UpdateQuestion_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateQuestionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).UpdateQuestion(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_UpdateQuestion_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).UpdateQuestion(ctx, req.(*UpdateQuestionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_QuestionDetail_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(QuestionDetailRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).QuestionDetail(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_QuestionDetail_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).QuestionDetail(ctx, req.(*QuestionDetailRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_LatestQuestions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(LatestQuestionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).LatestQuestions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_LatestQuestions_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).LatestQuestions(ctx, req.(*LatestQuestionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_AnswerList_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AnswerListRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).AnswerList(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_AnswerList_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).AnswerList(ctx, req.(*AnswerListRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_AddAnswer_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AddAnswerRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).AddAnswer(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_AddAnswer_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).AddAnswer(ctx, req.(*AddAnswerRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_DeleteAnswer_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteAnswerRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).DeleteAnswer(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_DeleteAnswer_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).DeleteAnswer(ctx, req.(*DeleteAnswerRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_UpdateAnswer_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateAnswerRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).UpdateAnswer(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_UpdateAnswer_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).UpdateAnswer(ctx, req.(*UpdateAnswerRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_AnswerAgree_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AnswerAgreeRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).AnswerAgree(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_AnswerAgree_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).AnswerAgree(ctx, req.(*AnswerAgreeRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _QAService_QuestionReadCount_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(QuestionReadCountRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QAServiceServer).QuestionReadCount(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: QAService_QuestionReadCount_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QAServiceServer).QuestionReadCount(ctx, req.(*QuestionReadCountRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// QAService_ServiceDesc is the grpc.ServiceDesc for QAService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var QAService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "qa.QAService",
	HandlerType: (*QAServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "UserLogin",
			Handler:    _QAService_UserLogin_Handler,
		},
		{
			MethodName: "UserLogout",
			Handler:    _QAService_UserLogout_Handler,
		},
		{
			MethodName: "UserRegister",
			Handler:    _QAService_UserRegister_Handler,
		},
		{
			MethodName: "AddQuestion",
			Handler:    _QAService_AddQuestion_Handler,
		},
		{
			MethodName: "DeleteQuestion",
			Handler:    _QAService_DeleteQuestion_Handler,
		},
		{
			MethodName: "UpdateQuestion",
			Handler:    _QAService_UpdateQuestion_Handler,
		},
		{
			MethodName: "QuestionDetail",
			Handler:    _QAService_QuestionDetail_Handler,
		},
		{
			MethodName: "LatestQuestions",
			Handler:    _QAService_LatestQuestions_Handler,
		},
		{
			MethodName: "AnswerList",
			Handler:    _QAService_AnswerList_Handler,
		},
		{
			MethodName: "AddAnswer",
			Handler:    _QAService_AddAnswer_Handler,
		},
		{
			MethodName: "DeleteAnswer",
			Handler:    _QAService_DeleteAnswer_Handler,
		},
		{
			MethodName: "UpdateAnswer",
			Handler:    _QAService_UpdateAnswer_Handler,
		},
		{
			MethodName: "AnswerAgree",
			Handler:    _QAService_AnswerAgree_Handler,
		},
		{
			MethodName: "QuestionReadCount",
			Handler:    _QAService_QuestionReadCount_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "qa.proto",
}
