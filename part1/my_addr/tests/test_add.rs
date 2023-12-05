use my_addr;

#[test]
fn test_add2() {
    assert_eq!(my_addr::add2(1, 2), 3);
}

use test::Bencher;
#[bench]
fn bench_add2(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(my_addr::add2(2, 2), 4);
    })
}
