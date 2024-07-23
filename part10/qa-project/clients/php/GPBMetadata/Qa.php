<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace GPBMetadata;

class Qa
{
    public static $is_initialized = false;

    public static function initOnce() {
        $pool = \Google\Protobuf\Internal\DescriptorPool::getGeneratedPool();

        if (static::$is_initialized == true) {
          return;
        }
        \GPBMetadata\Google\Api\Annotations::initOnce();
        $pool->internalAddGeneratedFile(
            '
�
qa.protoqa"6
UserLoginRequest
username (	
password (	"
UserLoginReply
token (	""
UserLogoutRequest
token (	" 
UserLogoutReply
state ("9
UserRegisterRequest
username (	
password (	""
UserRegisterReply
state ("7
VerifyTokenRequest
token (	

request_id (	"C
VerifyTokenReply
state (
reason (	
username (	"H
AddQuestionRequest
title (	
content (	

created_by (	"
AddQuestionReply

id ("5
DeleteQuestionRequest

id (
username (	"$
DeleteQuestionReply
state ("W
UpdateQuestionRequest

id (
title (	
content (	

updated_by (	"$
UpdateQuestionReply
state ("}
AnswerEntity

id (
question_id (
content (	

created_by (	
agree_count (

has_agreed ("4
AddAnswerRequest 
answer (2.qa.AnswerEntity"
AddAnswerReply

id ("3
DeleteAnswerRequest

id (
username (	""
DeleteAnswerReply
state ("D
UpdateAnswerRequest

id (
content (	
username (	""
UpdateAnswerReply
state ("D
AnswerAgreeRequest

id (

created_by (	
action (	"F
AnswerAgreeReply
state (
reason (	
agree_count ("5
QuestionDetailRequest

id (
username (	";
QuestionDetailReply$
question (2.qa.QuestionEntity"y
QuestionEntity

id (
title (	
content (	

created_by (	

read_count (
reply_count ("J
LatestQuestionsRequest
username (	
last_id (
limit ("Y
LatestQuestionsReply 
list (2.qa.QuestionEntity
last_id (
is_end ("W
AnswerListRequest
question_id (
page (
limit (
username (	"�
AnswerListReply
list (2.qa.AnswerEntity
total (

total_page (
	page_size (
current_page (
is_end ("3
AnswerDetailRequest

id (
username (	"5
AnswerDetailReply 
answer (2.qa.AnswerEntity2�
	QAServiceP
	UserLogin.qa.UserLoginRequest.qa.UserLoginReply"���"/v1/user_login:*8

UserLogout.qa.UserLogoutRequest.qa.UserLogoutReply>
UserRegister.qa.UserRegisterRequest.qa.UserRegisterReply;
VerifyToken.qa.VerifyTokenRequest.qa.VerifyTokenReply;
AddQuestion.qa.AddQuestionRequest.qa.AddQuestionReplyD
DeleteQuestion.qa.DeleteQuestionRequest.qa.DeleteQuestionReplyD
UpdateQuestion.qa.UpdateQuestionRequest.qa.UpdateQuestionReplyD
QuestionDetail.qa.QuestionDetailRequest.qa.QuestionDetailReplyG
LatestQuestions.qa.LatestQuestionsRequest.qa.LatestQuestionsReply8

AnswerList.qa.AnswerListRequest.qa.AnswerListReply5
	AddAnswer.qa.AddAnswerRequest.qa.AddAnswerReply>
DeleteAnswer.qa.DeleteAnswerRequest.qa.DeleteAnswerReply>
UpdateAnswer.qa.UpdateAnswerRequest.qa.UpdateAnswerReply>
AnswerDetail.qa.AnswerDetailRequest.qa.AnswerDetailReply;
AnswerAgree.qa.AnswerAgreeRequest.qa.AnswerAgreeReplyBZ.;pb�Qabproto3'
        , true);

        static::$is_initialized = true;
    }
}

