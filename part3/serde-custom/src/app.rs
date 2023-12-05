// 引入serde库
use serde::de::{self,MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// 用户信息
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i64,
    name: String,
    lang: String,

    #[serde(skip)]
    is_married: bool,

    #[serde(
        serialize_with = "serialize_hobbies",
        deserialize_with = "deserialize_hobbies"
    )]
    hobbies: Vec<String>,
    address: Address,
}

// 地址信息
#[derive(Serialize,Debug)]
struct Address {
    city: String,
    street: String,
    post_code: i32,
}

// 为结构体Address自定义deserialize方法
impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 为结构体字段实现自动反序列化小写机制
        #[derive(Debug)]
        // #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            City,
            Street,
            PostCode,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`city` or `street` or `post_code`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "city" => Ok(Field::City),
                            "street" => Ok(Field::Street),
                            "post_code" => Ok(Field::PostCode),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AddressVisitor;
        impl<'de> Visitor<'de> for AddressVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Address")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Address, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let city = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let street = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let post_code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Address{
                    city:city,
                    street:street,
                    post_code:post_code,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Address, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut city = None;
                let mut street = None;
                let mut post_code = None;
                while let Some(key) = map.next_key()? {
                    println!("key: {:?}",key);
                    match key {
                        Field::City => {
                            if city.is_some() {
                                return Err(de::Error::duplicate_field("city"));
                            }
                            city = Some(map.next_value()?);
                        }
                        Field::Street => {
                            if street.is_some() {
                                return Err(de::Error::duplicate_field("street"));
                            }
                            street = Some(map.next_value()?);
                        }
                        Field::PostCode => {
                            if post_code.is_some() {
                                return Err(de::Error::duplicate_field("post_code"));
                            }
                            post_code = Some(map.next_value()?);
                        }
                    }
                }

                let city = city.ok_or_else(|| de::Error::missing_field("city"))?;
                let street = street.ok_or_else(|| de::Error::missing_field("street"))?;
                let post_code = post_code.ok_or_else(|| de::Error::missing_field("street"))?;
                Ok(Address{
                    city:city,
                    street:street,
                    post_code:post_code,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["city", "street","post_code"];
        deserializer.deserialize_struct("Address", FIELDS, AddressVisitor)

    }
}

fn serialize_hobbies<S>(hobbies: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(hobbies.join(",").as_str())
}

// 返回Vec<String>和对应的错误error
fn deserialize_hobbies<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let v: Vec<&str> = s.split(",").collect();
    let mut arr = Vec::new();
    for val in v {
        arr.push(val.to_string())
    }
    Ok(arr)
}

fn main() {
    let p = Person {
        id: 1,
        name: "daheige".to_string(),
        lang: "rust".to_string(),
        is_married: true,
        hobbies: vec!["reading".to_string(), "music".to_string()],
        address: Address {
            city: "shenzhen".to_string(),
            street: "guangming".to_string(),
            post_code: 518000,
        },
    };
    let s = serde_json::to_string(&p).unwrap();
    println!("json encode person encode to str: {}", s);

    let p: Person = serde_json::from_str(&s).unwrap();
    println!("json decode person:{:?}", p);
    println!("person id:{},name:{} hobbies:{:?},city:{}", p.id, p.name, p.hobbies,p.address.city);
}
