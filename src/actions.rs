pub enum Action {
    SkipTurn,
    EquipWeapon(usize),
    Heal,
    Attack,
    Research,
    Quit,
}

