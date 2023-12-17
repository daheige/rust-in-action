#![feature(test)]
use my_addr;

#[test]
fn test_add2() {
    assert_eq!(my_addr::add2(1, 2), 3);
    println!("add2(1,2) = {}",my_addr::add2(1, 2));
}

use test::Bencher;
#[bench]
fn bench_add2(b: &mut Bencher){
    b.iter(|| {
        for i in 1..100{
            println!("add2(1,{}) = {}",i,add2(1,i));
        }
    })
}
