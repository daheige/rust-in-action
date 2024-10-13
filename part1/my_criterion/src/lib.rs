// 实现n的阶乘，也就是1*2*3一直乘以到n的结果
pub fn factorial(n:u32)->u32{
    if n == 1 || n == 0{
        return n;
    }

    n *factorial(n-1)
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(3);
        println!("result:{}",result);
        assert_eq!(result, 6);
    }
}
