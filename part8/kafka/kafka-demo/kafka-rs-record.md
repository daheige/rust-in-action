# kafka::producer::Record定义

kafka-rust底层kafka::producer::Record是一个结构体，它包含了kafka消息的key、value、topic、patition等基本字段，核心代码定义如下：

```rust
pub struct Record<'a, K, V> {
    /// Key data of this (message) record.
    pub key: K,
    /// Value data of this (message) record.
    pub value: V,
    /// Name of the topic this message is supposed to be delivered to.
    pub topic: &'a str,

    /// The partition id of the topic to deliver this message to.
    /// This partition may be `< 0` in which case it is considered
    /// "unspecified". A `Producer` will then typically try to derive
    /// a partition on its own.
    pub partition: i32,
}

impl<'a, K, V> Record<'a, K, V> {
    /// Convenience function to create a new key/value record with an
    /// "unspecified" partition - this is, a partition set to a negative
    /// value.
    #[inline]
    pub fn from_key_value(topic: &'a str, key: K, value: V) -> Record<'a, K, V> {
        Record {
            key,
            value,
            topic,
            partition: -1,
        }
    }

    /// Convenience method to set the partition.
    #[inline]
    pub fn with_partition(mut self, partition: i32) -> Self {
        self.partition = partition;
        self
    }
}

impl<'a, V> Record<'a, (), V> {
    /// Convenience function to create a new value only record with an
    /// "unspecified" partition - this is, a partition set to a negative
    /// value.
    #[inline]
    pub fn from_value(topic: &'a str, value: V) -> Record<'a, (), V> {
        Record {
            key: (),
            value,
            topic,
            partition: -1,
        }
    }
}
```

从上述代码片段可以看出，使用Record::from_key_value关联函数可以创建一个Record实例或者直接初始化Record实例。
该Record实例的引用作为producer.send方法的参数，用来发送kafka消息。
