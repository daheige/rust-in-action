<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 更新回答返回结果
 *
 * Generated from protobuf message <code>qa.UpdateAnswerReply</code>
 */
class UpdateAnswerReply extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>int64 state = 1;</code>
     */
    protected $state = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $state
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>int64 state = 1;</code>
     * @return int|string
     */
    public function getState()
    {
        return $this->state;
    }

    /**
     * Generated from protobuf field <code>int64 state = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setState($var)
    {
        GPBUtil::checkInt64($var);
        $this->state = $var;

        return $this;
    }

}

