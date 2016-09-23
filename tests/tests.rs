extern crate impl_bug;
use impl_bug::S;

#[test]
fn it_works_integration() {
    let s = S {};
    let hi: String = "hi".into();
    let there: String = "there".into();

    let v = vec![Some(&hi), None, Some(&there)];
    let ret: Vec<&String> = s.return_iter(&v).collect();
    let exp: Vec<&String> = vec![&hi, &there];
    assert_eq!(ret, exp);
}
