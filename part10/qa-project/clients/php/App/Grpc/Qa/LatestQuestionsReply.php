<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 最新问题列表返回结果
 *
 * Generated from protobuf message <code>qa.LatestQuestionsReply</code>
 */
class LatestQuestionsReply extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>repeated .qa.QuestionEntity list = 1;</code>
     */
    private $list;
    /**
     * 上一次的question_id
     *
     * Generated from protobuf field <code>uint64 last_id = 2;</code>
     */
    protected $last_id = 0;
    /**
     * 是否到底了
     *
     * Generated from protobuf field <code>bool is_end = 3;</code>
     */
    protected $is_end = false;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type array<\App\Grpc\Qa\QuestionEntity>|\Google\Protobuf\Internal\RepeatedField $list
     *     @type int|string $last_id
     *           上一次的question_id
     *     @type bool $is_end
     *           是否到底了
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>repeated .qa.QuestionEntity list = 1;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getList()
    {
        return $this->list;
    }

    /**
     * Generated from protobuf field <code>repeated .qa.QuestionEntity list = 1;</code>
     * @param array<\App\Grpc\Qa\QuestionEntity>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setList($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \App\Grpc\Qa\QuestionEntity::class);
        $this->list = $arr;

        return $this;
    }

    /**
     * 上一次的question_id
     *
     * Generated from protobuf field <code>uint64 last_id = 2;</code>
     * @return int|string
     */
    public function getLastId()
    {
        return $this->last_id;
    }

    /**
     * 上一次的question_id
     *
     * Generated from protobuf field <code>uint64 last_id = 2;</code>
     * @param int|string $var
     * @return $this
     */
    public function setLastId($var)
    {
        GPBUtil::checkUint64($var);
        $this->last_id = $var;

        return $this;
    }

    /**
     * 是否到底了
     *
     * Generated from protobuf field <code>bool is_end = 3;</code>
     * @return bool
     */
    public function getIsEnd()
    {
        return $this->is_end;
    }

    /**
     * 是否到底了
     *
     * Generated from protobuf field <code>bool is_end = 3;</code>
     * @param bool $var
     * @return $this
     */
    public function setIsEnd($var)
    {
        GPBUtil::checkBool($var);
        $this->is_end = $var;

        return $this;
    }

}

