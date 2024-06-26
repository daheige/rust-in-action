<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 问题阅读数处理的返回结果
 *
 * Generated from protobuf message <code>qa.QuestionReadCountReply</code>
 */
class QuestionReadCountReply extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>int64 id = 1;</code>
     */
    protected $id = 0;
    /**
     * Generated from protobuf field <code>int64 read_count = 2;</code>
     */
    protected $read_count = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $id
     *     @type int|string $read_count
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
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
     * Generated from protobuf field <code>int64 read_count = 2;</code>
     * @return int|string
     */
    public function getReadCount()
    {
        return $this->read_count;
    }

    /**
     * Generated from protobuf field <code>int64 read_count = 2;</code>
     * @param int|string $var
     * @return $this
     */
    public function setReadCount($var)
    {
        GPBUtil::checkInt64($var);
        $this->read_count = $var;

        return $this;
    }

}

