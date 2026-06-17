use crate::items::Item;
use rand::Rng;


pub struct Player {
    hp: i32,
    max_hp: i32,
    weapon: Option<Item>,
    pub inventory: Vec<Item>,
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        let mut player = Player {
            hp: 100,
            max_hp: 100,
            weapon: None,
            inventory: Vec::with_capacity(10),
            x: 3,
            y: 3,
        };

        player.inventory.push(Item::give_sword("basic"));
        player
    }

    pub fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Your Inventory is empty!!");
            return;
        }
        println!("Your inventory:");

        let mut i = 1;
        for item in &self.inventory {
            print!("{i}. ");
            item.print();
            i += 1;
        }
        print!("\n");
    }

    pub fn equip_weapon(&mut self, index: usize) {
        let len = self.inventory.len();

        assert!(
            index < len,
            "No such index {} in inventory of len {}",
            index,
            len
        );

        if let Some(weapon) = self.weapon.take() {
            self.inventory.push(weapon);
            self.inventory.swap(len - 1, index);
        }
        self.weapon = self.inventory.pop();

        print!("New weapon [");
        self.weapon.as_ref().unwrap().print();
        print!("] equipped!\n");
    }
}
