use crate::map::Cell::Spawner;

enum Cell {
    Floor,
    Wall,
    Corridor,
    Player,
    Barrel,
    Enemy,
    Void,
    Spawner(u8),
}

pub struct Map {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

fn can_stand(cell: &Cell) -> bool {
    match cell {
        Cell::Floor => true,
        _ => false,
    }
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            data: Vec::with_capacity(8),
            width: 10,
            height: 15,
        };
        for i in 0..map.width {
            map.data.push(Vec::with_capacity(9));
            for _ in 0..map.height {
                map.data[i].push(Cell::Floor);
            }
        }
        for i in 0..map.width {
            for j in 0..map.height {
                if ((i == 1 || i == map.width - 2) && j > 0 && j < map.height - 1) ||
                        ((j == 1 || j == map.height - 2) && i > 0 && i < map.width - 1) {
                    map.data[i][j] = Cell::Wall;
                }
                if i == 0 || j == 0 || i == map.width - 1 || j == map.height - 1 {
                    map.data[i][j] = Cell::Void;
                }
            }
        }
        map
    }

    pub fn print(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let form = match &self.data[i][j] {
                    Cell::Floor => '.',
                    Cell::Corridor => '=',
                    Cell::Wall => '#',
                    Cell::Player => '@',
                    Cell::Void => ' ',
                    Cell::Spawner(power) => ('0' as u8 + power) as char,
                    Cell::Enemy => 'E',
                    Cell::Barrel => 'B',
                };
                print!("{form}");
            }
            print!("\n");
        }
    }

    pub fn place_player(&mut self, coords: (usize, usize)) {
        self.data[coords.0][coords.1] = Cell::Player;
    }

    pub fn place_spawner(&mut self, coords: (usize, usize), power: u8) {
        self.data[coords.0][coords.1] = Cell::Spawner(power);
    }

    pub fn place_enemy(&mut self, coords: (usize, usize)) {
        self.data[coords.0][coords.1] = Cell::Enemy;
    }

    pub fn move_player(&mut self, source: (usize, usize), target: (usize, usize)) {
        self.data[source.0][source.1] = Cell::Floor;
        self.data[target.0][target.1] = Cell::Player;

    }

    pub fn can_move(&self, coords: (usize, usize)) -> bool {
        can_stand(&self.data[coords.0][coords.1])
    }


    pub fn cut_coords(&self, coords: (usize, usize)) -> (usize, usize) {
        let mut x = coords.0;
        let mut y = coords.1;
        if x < 2 {
            x = 2;
        }
        if y < 2 {
            y = 2;
        }
        if x > self.width - 3 {
            x = self.width - 3;
        }
        if y > self.height - 3 {
            y = self.height - 3;
        }
        (x, y)
    }
}

