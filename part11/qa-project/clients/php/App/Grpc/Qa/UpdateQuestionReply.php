<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 更新问题返回结果
 *
 * Generated from protobuf message <code>qa.UpdateQuestionReply</code>
 */
class UpdateQuestionReply extends \Google\Protobuf\Internal\Message
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

