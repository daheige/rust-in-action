<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 登录返回结果
 *
 * Generated from protobuf message <code>qa.UserLoginReply</code>
 */
class UserLoginReply extends \Google\Protobuf\Internal\Message
{
    /**
     * 登录成功后加密的token
     *
     * Generated from protobuf field <code>string token = 1;</code>
     */
    protected $token = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $token
     *           登录成功后加密的token
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * 登录成功后加密的token
     *
     * Generated from protobuf field <code>string token = 1;</code>
     * @return string
     */
    public function getToken()
    {
        return $this->token;
    }

    /**
     * 登录成功后加密的token
     *
     * Generated from protobuf field <code>string token = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setToken($var)
    {
        GPBUtil::checkString($var, True);
        $this->token = $var;

        return $this;
    }

}

