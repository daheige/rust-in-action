<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Qa;

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
     * @param \Qa\UserLoginRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserLogin(\Qa\UserLoginRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserLogin',
        $argument,
        ['\Qa\UserLoginReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户退出
     * @param \Qa\UserLogoutRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserLogout(\Qa\UserLogoutRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserLogout',
        $argument,
        ['\Qa\UserLogoutReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户注册
     * @param \Qa\UserRegisterRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UserRegister(\Qa\UserRegisterRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UserRegister',
        $argument,
        ['\Qa\UserRegisterReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 验证登录的token是否有效
     * @param \Qa\VerifyTokenRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function VerifyToken(\Qa\VerifyTokenRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/VerifyToken',
        $argument,
        ['\Qa\VerifyTokenReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 发表问题
     * @param \Qa\AddQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddQuestion(\Qa\AddQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AddQuestion',
        $argument,
        ['\Qa\AddQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 删除问题
     * @param \Qa\DeleteQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteQuestion(\Qa\DeleteQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/DeleteQuestion',
        $argument,
        ['\Qa\DeleteQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 修改问题
     * @param \Qa\UpdateQuestionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateQuestion(\Qa\UpdateQuestionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UpdateQuestion',
        $argument,
        ['\Qa\UpdateQuestionReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 查看问题详情
     * @param \Qa\QuestionDetailRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function QuestionDetail(\Qa\QuestionDetailRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/QuestionDetail',
        $argument,
        ['\Qa\QuestionDetailReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 最新问题列表（采用下拉分页形式获取数据，按照id desc倒序）
     * @param \Qa\LatestQuestionsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function LatestQuestions(\Qa\LatestQuestionsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/LatestQuestions',
        $argument,
        ['\Qa\LatestQuestionsReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 回答列表
     * @param \Qa\AnswerListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerList(\Qa\AnswerListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerList',
        $argument,
        ['\Qa\AnswerListReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 添加问题回答
     * @param \Qa\AddAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddAnswer(\Qa\AddAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AddAnswer',
        $argument,
        ['\Qa\AddAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 删除问题对应的回答
     * @param \Qa\DeleteAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteAnswer(\Qa\DeleteAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/DeleteAnswer',
        $argument,
        ['\Qa\DeleteAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 修改回答
     * @param \Qa\UpdateAnswerRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateAnswer(\Qa\UpdateAnswerRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/UpdateAnswer',
        $argument,
        ['\Qa\UpdateAnswerReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 查看答案详情
     * @param \Qa\AnswerDetailRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerDetail(\Qa\AnswerDetailRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerDetail',
        $argument,
        ['\Qa\AnswerDetailReply', 'decode'],
        $metadata, $options);
    }

    /**
     * 用户点赞回答和取消点赞
     * @param \Qa\AnswerAgreeRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AnswerAgree(\Qa\AnswerAgreeRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/qa.QAService/AnswerAgree',
        $argument,
        ['\Qa\AnswerAgreeReply', 'decode'],
        $metadata, $options);
    }

}
