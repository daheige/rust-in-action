<?php
// GENERATED CODE -- DO NOT EDIT!

namespace App\Grpc\Qa;

/**
 * qa服务接口定义
 */
class QAServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * 用户登录
     * @param \App\Grpc\Qa\UserLoginRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserLogin(\App\Grpc\Qa\UserLoginRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserLogin',
        $argument,
        ['\App\Grpc\Qa\UserLoginReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户退出
     * @param \App\Grpc\Qa\UserLogoutRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserLogout(\App\Grpc\Qa\UserLogoutRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserLogout',
        $argument,
        ['\App\Grpc\Qa\UserLogoutReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户注册
     * @param \App\Grpc\Qa\UserRegisterRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserRegister(\App\Grpc\Qa\UserRegisterRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserRegister',
        $argument,
        ['\App\Grpc\Qa\UserRegisterReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 验证登录的token是否有效
     * @param \App\Grpc\Qa\VerifyTokenRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function VerifyToken(\App\Grpc\Qa\VerifyTokenRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/VerifyToken',
        $argument,
        ['\App\Grpc\Qa\VerifyTokenReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 发表问题
     * @param \App\Grpc\Qa\AddQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddQuestion(\App\Grpc\Qa\AddQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AddQuestion',
        $argument,
        ['\App\Grpc\Qa\AddQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 删除问题
     * @param \App\Grpc\Qa\DeleteQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteQuestion(\App\Grpc\Qa\DeleteQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/DeleteQuestion',
        $argument,
        ['\App\Grpc\Qa\DeleteQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 修改问题
     * @param \App\Grpc\Qa\UpdateQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateQuestion(\App\Grpc\Qa\UpdateQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UpdateQuestion',
        $argument,
        ['\App\Grpc\Qa\UpdateQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 查看问题详情
     * @param \App\Grpc\Qa\QuestionDetailRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function QuestionDetail(\App\Grpc\Qa\QuestionDetailRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/QuestionDetail',
        $argument,
        ['\App\Grpc\Qa\QuestionDetailReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 最新问题列表（采用下拉分页形式获取数据，按照id desc倒序）
     * @param \App\Grpc\Qa\LatestQuestionsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function LatestQuestions(\App\Grpc\Qa\LatestQuestionsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/LatestQuestions',
        $argument,
        ['\App\Grpc\Qa\LatestQuestionsReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 回答列表
     * @param \App\Grpc\Qa\AnswerListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerList(\App\Grpc\Qa\AnswerListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerList',
        $argument,
        ['\App\Grpc\Qa\AnswerListReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 添加问题回答
     * @param \App\Grpc\Qa\AddAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddAnswer(\App\Grpc\Qa\AddAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AddAnswer',
        $argument,
        ['\App\Grpc\Qa\AddAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 删除问题对应的回答
     * @param \App\Grpc\Qa\DeleteAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteAnswer(\App\Grpc\Qa\DeleteAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/DeleteAnswer',
        $argument,
        ['\App\Grpc\Qa\DeleteAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 修改回答
     * @param \App\Grpc\Qa\UpdateAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateAnswer(\App\Grpc\Qa\UpdateAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UpdateAnswer',
        $argument,
        ['\App\Grpc\Qa\UpdateAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 查看答案详情
     * @param \App\Grpc\Qa\AnswerDetailRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerDetail(\App\Grpc\Qa\AnswerDetailRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerDetail',
        $argument,
        ['\App\Grpc\Qa\AnswerDetailReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户点赞回答和取消点赞
     * @param \App\Grpc\Qa\AnswerAgreeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerAgree(\App\Grpc\Qa\AnswerAgreeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerAgree',
        $argument,
        ['\App\Grpc\Qa\AnswerAgreeReply', 'decode'],
        $metadata, $options);
    }

}
