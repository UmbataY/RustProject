use crate::player::player::Player;

#[test]
fn test_create_player() {
    let player = Player::new();

    assert_eq!(player.get_level(), 0);
    assert_eq!(player.get_skill_points(), 0);
    assert_eq!(player.get_gold(), 0);

    assert_eq!(player.get_strength(), 1);
    assert_eq!(player.get_agility(), 1);
    assert_eq!(player.get_vitality(), 30);
}

#[test]
fn test_increase_vitality() {
    let mut player = Player::new();
    player.increase_vitality();

    assert_eq!(player.get_vitality(), 31);
}

#[test]
fn test_increase_agility() {
    let mut player = Player::new();
    player.increase_agility();

    assert_eq!(player.get_agility(), 2);
}

#[test]
fn test_increase_strength() {
    let mut player = Player::new();
    player.increase_strength();

    assert_eq!(player.get_strength(), 2);
}

#[test]
fn test_increase_level() {
    let mut player = Player::new();
    player.increase_level();

    assert_eq!(player.get_level(), 1);
}

#[test]
fn test_add_gold() {
    let mut player = Player::new();
    player.add_gold(100);

    assert_eq!(player.get_gold(), 100);
}

#[test]
fn test_add_skill_points() {
    let mut player = Player::new();
    player.add_skillpoints(2);

    assert_eq!(player.get_skill_points(), 2);
}