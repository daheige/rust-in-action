<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 验证登录token请求
 *
 * Generated from protobuf message <code>qa.VerifyTokenRequest</code>
 */
class VerifyTokenRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * 登录成功后的token
     *
     * Generated from protobuf field <code>string token = 1;</code>
     */
    protected $token = '';
    /**
     * 请求id
     *
     * Generated from protobuf field <code>string request_id = 2;</code>
     */
    protected $request_id = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $token
     *           登录成功后的token
     *     @type string $request_id
     *           请求id
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * 登录成功后的token
     *
     * Generated from protobuf field <code>string token = 1;</code>
     * @return string
     */
    public function getToken()
    {
        return $this->token;
    }

    /**
     * 登录成功后的token
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

    /**
     * 请求id
     *
     * Generated from protobuf field <code>string request_id = 2;</code>
     * @return string
     */
    public function getRequestId()
    {
        return $this->request_id;
    }

    /**
     * 请求id
     *
     * Generated from protobuf field <code>string request_id = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setRequestId($var)
    {
        GPBUtil::checkString($var, True);
        $this->request_id = $var;

        return $this;
    }

}

