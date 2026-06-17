enum EnemyType {
    Mouse,
}

pub struct Enemy {
    hp: i32,
    max_hp: i32,
    damage: i32,
    enemy_type: EnemyType,
    x: i32,
    y: i32,
}

impl Enemy {
    pub fn spawn(x: i32, y: i32, enemy_type: &str) -> Self {
        match enemy_type {
            "mouse" | _ => Enemy {
                hp: 20,
                max_hp: 20,
                damage: 4,
                enemy_type: EnemyType::Mouse,
                x: x,
                y: y,
            },
        }
    }
}
