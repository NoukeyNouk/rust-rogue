enum Cell {
    Floor,
    Wall,
    Corridor,
    Player,
    Barrel,
    Enemy,
    Void,
}

pub struct Map {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
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
            for j in 0..map.height {
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
                    _ => '?',
                };
                print!("{form}");
            }
            print!("\n");
        }
    }

    pub fn place_player(&mut self, coords: (usize, usize)) {
        self.data[coords.0][coords.1] = Cell::Player;
    }

    pub fn can_move(&self, x: usize, y: usize) -> bool {
        if x < 2 || y < 2 {
            return false;
        }
        if x > self.width - 3 || y > self.height - 3 {
            return false;
        }
        true
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
