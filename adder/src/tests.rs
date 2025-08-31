
use super::*;
use shapes::Rectangle;

#[test]
fn exploration() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
#[should_panic(expected = "fails")]
fn another() {
    panic!("This fails but only substrig matters");
}

#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };

    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}
