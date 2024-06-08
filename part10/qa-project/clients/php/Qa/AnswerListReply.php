<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 回答列表返回结果
 *
 * Generated from protobuf message <code>qa.AnswerListReply</code>
 */
class AnswerListReply extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>repeated .qa.AnswerEntity list = 1;</code>
     */
    private $list;
    /**
     * Generated from protobuf field <code>int64 total = 2;</code>
     */
    protected $total = 0;
    /**
     * Generated from protobuf field <code>int64 total_page = 3;</code>
     */
    protected $total_page = 0;
    /**
     * Generated from protobuf field <code>bool is_end = 4;</code>
     */
    protected $is_end = false;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type array<\Qa\AnswerEntity>|\Google\Protobuf\Internal\RepeatedField $list
     *     @type int|string $total
     *     @type int|string $total_page
     *     @type bool $is_end
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>repeated .qa.AnswerEntity list = 1;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getList()
    {
        return $this->list;
    }

    /**
     * Generated from protobuf field <code>repeated .qa.AnswerEntity list = 1;</code>
     * @param array<\Qa\AnswerEntity>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setList($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \Qa\AnswerEntity::class);
        $this->list = $arr;

        return $this;
    }

    /**
     * Generated from protobuf field <code>int64 total = 2;</code>
     * @return int|string
     */
    public function getTotal()
    {
        return $this->total;
    }

    /**
     * Generated from protobuf field <code>int64 total = 2;</code>
     * @param int|string $var
     * @return $this
     */
    public function setTotal($var)
    {
        GPBUtil::checkInt64($var);
        $this->total = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>int64 total_page = 3;</code>
     * @return int|string
     */
    public function getTotalPage()
    {
        return $this->total_page;
    }

    /**
     * Generated from protobuf field <code>int64 total_page = 3;</code>
     * @param int|string $var
     * @return $this
     */
    public function setTotalPage($var)
    {
        GPBUtil::checkInt64($var);
        $this->total_page = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>bool is_end = 4;</code>
     * @return bool
     */
    public function getIsEnd()
    {
        return $this->is_end;
    }

    /**
     * Generated from protobuf field <code>bool is_end = 4;</code>
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

