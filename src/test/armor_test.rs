use crate::armor::armor::Armor;

#[test]
fn test_armor_create_price() {
    let armor = Armor::new(1);

    assert_eq!(armor.get_price(), 100);
}


#[test]
fn test_armor_deal_damage() {
    let mut armor = Armor::new(1);
    let previous_strength = armor.get_strength();

    armor.deal_damage(2);

    assert_eq!(armor.get_strength(), previous_strength - 2);
}