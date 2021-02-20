use crate::action::Action;

#[test]
fn test_action_create_head_right() {
    let action = Action::new(String::from("head"), String::from("right"));
    assert_eq!(action.get_side(), 0);
    assert_eq!(action.get_place(), 0);
}

#[test]
fn test_action_create_legs_left() {
    let action = Action::new(String::from("legs"), String::from("left"));
    assert_eq!(action.get_side(), 1);
    assert_eq!(action.get_place(), 2);
}

#[test]
fn test_action_create_body_right() {
    let action = Action::new(String::from("body"), String::from("right"));
    assert_eq!(action.get_side(), 0);
    assert_eq!(action.get_place(), 1);
}

#[test]
fn test_action_create_head_left() {
    let action = Action::new(String::from("head"), String::from("left"));
    assert_eq!(action.get_side(), 1);
    assert_eq!(action.get_place(), 0);
}

