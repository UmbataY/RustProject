use crate::weapon::weapon::Weapon;

#[test]
fn test_armor_create_price() {
    let weapon = Weapon::new(1);

    assert_eq!(weapon.get_price(), 100);
}