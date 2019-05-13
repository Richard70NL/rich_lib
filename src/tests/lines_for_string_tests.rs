/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

use crate::lines::*;
use crate::tests::helper::*;

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

#[test]
fn count_test() {
    let s = create_string();
    assert_eq!(s.count(), 10);

    let s = String::new();
    assert_eq!(s.count(), 0);
}

/************************************************************************************************/

#[test]
fn get_test() {
    let s = create_string();
    assert_eq!(s.get(0), "line0");
    assert_eq!(s.get(5), "line5");
    assert_eq!(s.get(9), "line9");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn get_panic_test() {
    let s = String::new();
    let _a = s.get(0);
}

/************************************************************************************************/

#[test]
fn set_test() {
    let s = create_string()
        .set(1, String::from("lineA"))
        .set(3, String::from("lineB"))
        .set(5, String::from("lineC"));

    let mut s = s.set(7, String::from("lineD"));

    s = s.set(9, String::from("lineE"));

    assert_eq!(s.get(0), "line0");
    assert_eq!(s.get(1), "lineA");
    assert_eq!(s.get(2), "line2");
    assert_eq!(s.get(3), "lineB");
    assert_eq!(s.get(4), "line4");
    assert_eq!(s.get(5), "lineC");
    assert_eq!(s.get(6), "line6");
    assert_eq!(s.get(7), "lineD");
    assert_eq!(s.get(8), "line8");
    assert_eq!(s.get(9), "lineE");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn set_panic_test() {
    let s = String::new();
    let _s = s.set(5, String::from("line1"));
}

/************************************************************************************************/

#[test]
fn insert_test() {
    let mut s = create_string();

    s = s
        .insert(7, String::from("lineA"))
        .insert(5, String::from("lineB"));

    s = s.insert(3, String::from("lineC"));

    assert_eq!(s.get(0), "line0");
    assert_eq!(s.get(1), "line1");
    assert_eq!(s.get(2), "line2");
    assert_eq!(s.get(3), "lineC");
    assert_eq!(s.get(4), "line3");
    assert_eq!(s.get(5), "line4");
    assert_eq!(s.get(6), "lineB");
    assert_eq!(s.get(7), "line5");
    assert_eq!(s.get(8), "line6");
    assert_eq!(s.get(9), "lineA");
    assert_eq!(s.get(10), "line7");
    assert_eq!(s.get(11), "line8");
    assert_eq!(s.get(12), "line9");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn insert_panic_test() {
    let s = String::new();
    let _s = s.insert(5, String::from("line5"));
}

/************************************************************************************************/

#[test]
fn remove_test() {
    let mut s = create_string().remove(7);

    s = s.remove(5).remove(5).remove(2);

    assert_eq!(s.get(0), "line0");
    assert_eq!(s.get(1), "line1");
    assert_eq!(s.get(2), "line3");
    assert_eq!(s.get(3), "line4");
    assert_eq!(s.get(4), "line8");
    assert_eq!(s.get(5), "line9");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn remove_panic_test() {
    let s = String::new();
    let _s = s.remove(5);
}

/************************************************************************************************/

#[test]
fn append_test() {
    let mut s = String::new()
        .append(String::from("line0"))
        .append(String::from("line1"));
    s = s
        .append(String::from("line2"))
        .append(String::from("line3"));

    assert_eq!(s.get(0), "line0");
    assert_eq!(s.get(1), "line1");
    assert_eq!(s.get(2), "line2");
    assert_eq!(s.get(3), "line3");
}

/************************************************************************************************/

#[test]
fn prepend_test() {
    let mut s = String::new()
        .prepend(String::from("line0"))
        .prepend(String::from("line1"));
    s = s
        .prepend(String::from("line2"))
        .prepend(String::from("line3"));

    assert_eq!(s.get(0), "line3");
    assert_eq!(s.get(1), "line2");
    assert_eq!(s.get(2), "line1");
    assert_eq!(s.get(3), "line0");
}

/************************************************************************************************/

#[test]
fn first_test() {
    let s = create_string();
    assert_eq!(s.first(), "line0");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn first_panic_test() {
    let s = String::new();
    let _s = s.first();
}

/************************************************************************************************/

#[test]
fn last_test() {
    let s = create_string();
    assert_eq!(s.last(), "line9");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn last_panic_test() {
    let s = String::new();
    let _s = s.last();
}

/************************************************************************************************/

#[test]
fn remove_first_test() {
    let mut s = create_string().remove_first();
    assert_eq!(s.first(), "line1");

    s = s.remove_first().remove_first();
    assert_eq!(s.first(), "line3");
}

/************************************************************************************************/

#[test]
#[should_panic]
fn remove_first_panic_test() {
    let s = String::new();
    let _s = s.remove_first();
}

/************************************************************************************************/

#[test]
fn remove_last_test() {
    let s = create_string().remove_last();
    assert_eq!(s.last(), "line8");

    let s = s.remove_last().remove_last();
    assert_eq!(s.last(), "line6");
}
/************************************************************************************************/

#[test]
#[should_panic]
fn remove_last_panic_test() {
    let s = String::new();
    let _s = s.remove_last();
}

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/
