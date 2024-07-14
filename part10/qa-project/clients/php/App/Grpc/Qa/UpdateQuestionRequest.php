<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace App\Grpc\Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 更新问题请求
 *
 * Generated from protobuf message <code>qa.UpdateQuestionRequest</code>
 */
class UpdateQuestionRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 id = 1;</code>
     */
    protected $id = 0;
    /**
     * 标题
     *
     * Generated from protobuf field <code>string title = 2;</code>
     */
    protected $title = '';
    /**
     * 内容正文
     *
     * Generated from protobuf field <code>string content = 3;</code>
     */
    protected $content = '';
    /**
     * 更新者
     *
     * Generated from protobuf field <code>string updated_by = 4;</code>
     */
    protected $updated_by = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int|string $id
     *           问题id
     *     @type string $title
     *           标题
     *     @type string $content
     *           内容正文
     *     @type string $updated_by
     *           更新者
     * }
     */
    public function __construct($data = NULL) {
        \App\Grpc\GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 id = 1;</code>
     * @return int|string
     */
    public function getId()
    {
        return $this->id;
    }

    /**
     * 问题id
     *
     * Generated from protobuf field <code>uint64 id = 1;</code>
     * @param int|string $var
     * @return $this
     */
    public function setId($var)
    {
        GPBUtil::checkUint64($var);
        $this->id = $var;

        return $this;
    }

    /**
     * 标题
     *
     * Generated from protobuf field <code>string title = 2;</code>
     * @return string
     */
    public function getTitle()
    {
        return $this->title;
    }

    /**
     * 标题
     *
     * Generated from protobuf field <code>string title = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setTitle($var)
    {
        GPBUtil::checkString($var, True);
        $this->title = $var;

        return $this;
    }

    /**
     * 内容正文
     *
     * Generated from protobuf field <code>string content = 3;</code>
     * @return string
     */
    public function getContent()
    {
        return $this->content;
    }

    /**
     * 内容正文
     *
     * Generated from protobuf field <code>string content = 3;</code>
     * @param string $var
     * @return $this
     */
    public function setContent($var)
    {
        GPBUtil::checkString($var, True);
        $this->content = $var;

        return $this;
    }

    /**
     * 更新者
     *
     * Generated from protobuf field <code>string updated_by = 4;</code>
     * @return string
     */
    public function getUpdatedBy()
    {
        return $this->updated_by;
    }

    /**
     * 更新者
     *
     * Generated from protobuf field <code>string updated_by = 4;</code>
     * @param string $var
     * @return $this
     */
    public function setUpdatedBy($var)
    {
        GPBUtil::checkString($var, True);
        $this->updated_by = $var;

        return $this;
    }

}

