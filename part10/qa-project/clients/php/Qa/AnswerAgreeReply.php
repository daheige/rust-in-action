<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 回答点赞返回结果
 *
 * Generated from protobuf message <code>qa.AnswerAgreeReply</code>
 */
class AnswerAgreeReply extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>int64 id = 1;</code>
     */
    protected $id = 0;
    /**
     * Generated from protobuf field <code>int64 agree_count = 2;</code>
     */
    protected $agree_count = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $id
     *     @type int|string $agree_count
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>int64 id = 1;</code>
     * @return int|string
     */
    public function getId()
    {
        return $this->id;
    }

    /**
     * Generated from protobuf field <code>int64 id = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setId($var)
    {
        GPBUtil::checkInt64($var);
        $this->id = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>int64 agree_count = 2;</code>
     * @return int|string
     */
    public function getAgreeCount()
    {
        return $this->agree_count;
    }

    /**
     * Generated from protobuf field <code>int64 agree_count = 2;</code>
     * @param int|string $var
     * @return $this
     */
    public function setAgreeCount($var)
    {
        GPBUtil::checkInt64($var);
        $this->agree_count = $var;

        return $this;
    }

}
