<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: qa.proto

namespace Qa;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * 添加答案
 *
 * Generated from protobuf message <code>qa.AddAnswerRequest</code>
 */
class AddAnswerRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>.qa.AnswerEntity answer = 1;</code>
     */
    protected $answer = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Qa\AnswerEntity $answer
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Qa::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>.qa.AnswerEntity answer = 1;</code>
     * @return \Qa\AnswerEntity|null
     */
    public function getAnswer()
    {
        return $this->answer;
    }

    public function hasAnswer()
    {
        return isset($this->answer);
    }

    public function clearAnswer()
    {
        unset($this->answer);
    }

    /**
     * Generated from protobuf field <code>.qa.AnswerEntity answer = 1;</code>
     * @param \Qa\AnswerEntity $var
     * @return $this
     */
    public function setAnswer($var)
    {
        GPBUtil::checkMessage($var, \Qa\AnswerEntity::class);
        $this->answer = $var;

        return $this;
    }

}

