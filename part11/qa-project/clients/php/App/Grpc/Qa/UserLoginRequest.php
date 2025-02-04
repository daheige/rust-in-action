<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 登录请求
 *
 * Generated from protobuf message <code>qa.UserLoginRequest</code>
 */
class UserLoginRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * 用户名
     *
     * Generated from protobuf field <code>string username = 1;</code>
     */
    protected $username = '';
    /**
     * 访问密码
     *
     * Generated from protobuf field <code>string password = 2;</code>
     */
    protected $password = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $username
     *           用户名
     *     @type string $password
     *           访问密码
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * 用户名
     *
     * Generated from protobuf field <code>string username = 1;</code>
     * @return string
     */
    public function getUsername()
    {
        return $this->username;
    }

    /**
     * 用户名
     *
     * Generated from protobuf field <code>string username = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setUsername($var)
    {
        GPBUtil::checkString($var, True);
        $this->username = $var;

        return $this;
    }

    /**
     * 访问密码
     *
     * Generated from protobuf field <code>string password = 2;</code>
     * @return string
     */
    public function getPassword()
    {
        return $this->password;
    }

    /**
     * 访问密码
     *
     * Generated from protobuf field <code>string password = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setPassword($var)
    {
        GPBUtil::checkString($var, True);
        $this->password = $var;

        return $this;
    }

}

