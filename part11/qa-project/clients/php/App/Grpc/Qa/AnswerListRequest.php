<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 问题下的回答接口
 *
 * Generated from protobuf message <code>qa.AnswerListRequest</code>
 */
class AnswerListRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 question_id = 1;</code>
     */
    protected $question_id = 0;
    /**
     * 当前页数
     *
     * Generated from protobuf field <code>uint64 page = 2;</code>
     */
    protected $page = 0;
    /**
     * 每页数据
     *
     * Generated from protobuf field <code>uint64 limit = 3;</code>
     */
    protected $limit = 0;
    /**
     * 当前用户
     *
     * Generated from protobuf field <code>string username = 4;</code>
     */
    protected $username = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $question_id
     *           问题id
     *     @type int|string $page
     *           当前页数
     *     @type int|string $limit
     *           每页数据
     *     @type string $username
     *           当前用户
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 question_id = 1;</code>
     * @return int|string
     */
    public function getQuestionId()
    {
        return $this->question_id;
    }

    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 question_id = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setQuestionId($var)
    {
        GPBUtil::checkUint64($var);
        $this->question_id = $var;

        return $this;
    }

    /**
     * 当前页数
     *
     * Generated from protobuf field <code>uint64 page = 2;</code>
     * @return int|string
     */
    public function getPage()
    {
        return $this->page;
    }

    /**
     * 当前页数
     *
     * Generated from protobuf field <code>uint64 page = 2;</code>
     * @param int|string $var
     * @return $this
     */
    public function setPage($var)
    {
        GPBUtil::checkUint64($var);
        $this->page = $var;

        return $this;
    }

    /**
     * 每页数据
     *
     * Generated from protobuf field <code>uint64 limit = 3;</code>
     * @return int|string
     */
    public function getLimit()
    {
        return $this->limit;
    }

    /**
     * 每页数据
     *
     * Generated from protobuf field <code>uint64 limit = 3;</code>
     * @param int|string $var
     * @return $this
     */
    public function setLimit($var)
    {
        GPBUtil::checkUint64($var);
        $this->limit = $var;

        return $this;
    }

    /**
     * 当前用户
     *
     * Generated from protobuf field <code>string username = 4;</code>
     * @return string
     */
    public function getUsername()
    {
        return $this->username;
    }

    /**
     * 当前用户
     *
     * Generated from protobuf field <code>string username = 4;</code>
     * @param string $var
     * @return $this
     */
    public function setUsername($var)
    {
        GPBUtil::checkString($var, True);
        $this->username = $var;

        return $this;
    }

}

