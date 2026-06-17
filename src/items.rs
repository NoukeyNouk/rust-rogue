pub enum Item {
    SacDagger,
    Bow,
    HealingStaff(String, i32),
    Sword(String, i32),
}

impl Item {
    pub fn give_sac_dagger() -> Self {
        Item::SacDagger
    }

    pub fn give_bow() -> Self {
        Item::Bow
    }

    pub fn give_healing_staff() -> Self {
        Item::HealingStaff(String::from("Weakness destroyer"), 20)
    }

    pub fn give_sword() -> Self {
        Item::Sword(String::from("KILLer"), 30)
    }

    pub fn print(&self) {
        match self {
            Item::SacDagger => print!("Sac Dagger"),
            Item::Bow => print!("Bow"),
            Item::HealingStaff(name, power) => print!("{name} - {power} hp"),
            Item::Sword(name, power) => print!("{name} - {power} dmg"),
        }
    }
}
