const FIELD_WIDTH: usize = 8;
const FIELD_HEIGHT: usize = 8;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Place{
    VOID = 0,
    BLACK = 1,
    WHITE = 2
}

impl Place{
    pub fn opposite(self) -> Place{
        if self == Place::BLACK {
            return Place::WHITE;
        }
        else if self == Place::WHITE{
            return Place::BLACK;
        }
        else{
            return Place::VOID;
        }
    }
    pub fn player(self) -> Player{
        if self == Place::BLACK{
            return Player::BLACK;
        }
        else if self == Place::WHITE{
            return Player::WHITE;
        }
        panic!("void cannot convert to player");
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player{
    BLACK = 1,
    WHITE = 2
}

impl Player{
    pub fn enemy(self) -> Player{
        if self == Player::BLACK {
            return Player::WHITE;
        }
        else{
            return Player::BLACK;
        }
    }
    pub fn place(self) -> Place{
        if self == Player::BLACK{
            return Place::BLACK;
        }
        else{
            return Place::WHITE;
        }
    }
}

struct Field{
    fields: [[Place; FIELD_WIDTH]; FIELD_HEIGHT]
}

impl Field {
    fn new() -> Field{
        Field{ fields: [[Place::VOID; FIELD_WIDTH]; FIELD_HEIGHT]}
    }

    fn start(&mut self){
        let mut tmp_fields = [[Place::VOID; FIELD_WIDTH]; FIELD_HEIGHT];
        tmp_fields[3][3] = Place::BLACK;
        tmp_fields[4][4] = Place::BLACK;
        tmp_fields[3][4] = Place::WHITE;
        tmp_fields[4][3] = Place::WHITE;

        self.fields = tmp_fields;
    }

    fn at(&self, x: i8, y: i8) -> Place {
        self.fields[x as usize][y as usize]
    }

    fn is_able_to_place(&self, player: Player, x: i8, y: i8) -> bool{
        if self.at(x, y) != Place::VOID {
            return false;
        }

        return self.is_able_t_place_direction(player, x, y,  1, 1) ||
               self.is_able_t_place_direction(player, x, y,  1, 0) ||
               self.is_able_t_place_direction(player, x, y,  0, 1) ||
               self.is_able_t_place_direction(player, x, y,  1,-1) ||
               self.is_able_t_place_direction(player, x, y, -1, 1) ||
               self.is_able_t_place_direction(player, x, y,  0,-1) ||
               self.is_able_t_place_direction(player, x, y, -1, 0) ||
               self.is_able_t_place_direction(player, x, y, -1,-1);
    }

    fn is_able_t_place_direction(&self, player: Player, x: i8, y: i8, mx: i8, my: i8) -> bool {
        let mut found_enemy = false;
        let mut cx = x;
        let mut cy = y;
        loop {
            cx += mx;
            cy += my;
            if !Field::is_in_field(cx, cy) {
                return false;
            }

            let pos = self.at(cx, cy);

            if pos == Place::VOID {
                return false;
            }

            if pos == player.place() {
                return found_enemy
            }

            if pos == player.enemy().place(){
                found_enemy = true;
            }
        }
    }

    fn place(&mut self, player: Player, x: i8, y: i8) -> bool {
        if !self.is_able_to_place(player, x, y) {
            return false;
        }

        let dirs = [[0, 1], [1, 0], [1, 1], [1, -1], [-1, 1], [-1, 0], [0, -1], [-1, -1]];

        for dir in dirs.iter() {
            if self.is_able_t_place_direction(player, x, y, dir[0], dir[1]) {
                self.reverse_direction(player, x, y, dir[0], dir[1]);
            }
        }
        return true;
    }

    fn reverse_direction(&mut self, player: Player, x: i8, y: i8, mx: i8, my: i8) {
        if !self.is_able_t_place_direction(player, x, y, mx, my){
            return;
        }

        self.fields[x as usize][y as usize] = player.place();

        let mut cx = x;
        let mut cy = y;
        loop {
            cx += mx;
            cy += my;
            if !Field::is_in_field(cx, cy){
                return;
            }

            let pos = self.at(cx, cy);

            if pos == Place::VOID {
                return;
            }

            if pos == player.place() {
                return;
            }

            if pos == player.enemy().place(){
                self.fields[cx as usize][cy as usize] = player.place();
            }
        }
    }

    pub fn is_in_field(x: i8, y: i8) -> bool {
        x >= 0 && x < FIELD_WIDTH as i8 && y >= 0 && y < FIELD_HEIGHT as i8
    }

    fn print(&self) {
        print!("---------------------------------\n");
        for xs in self.fields.iter() {
            print!("|");
            for block in xs.iter() {
                let str = if *block == Place::BLACK {
                    "●"
                }
                else if *block == Place::WHITE {
                    "○"
                } else{
                    " "
                };

                print!(" {} |", str);
            }
            print!("\n");
            print!("---------------------------------\n");
        }
    }
}

fn main() {
    println!("Let's play reversi!");

    let mut field = Field::new();
    field.start();
    field.print();

    let mut player = Player::BLACK;
    loop{
        let xy = proc_input(&field, player);
        field.place(player, xy[0], xy[1]);
        field.print();
        player = player.enemy();
    }
}

//手番の入力を受け付け、置く座標を返す（実際に置けるかどうかの判断まではしない）
fn prompt(player: Player) -> [i8; 2] {
    if player == Player::BLACK {
        print!("BLACK");
    }
    else if player == Player::WHITE{
        print!("WHITE");
    }
    println!(": input x(1~8) and y(1~8) with whitespace separated (ex. \"1 1\") > ");

    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let mut ws = string.split_whitespace();
    let x: i8 = ws.next().unwrap_or("0").parse().unwrap_or(0);
    let y: i8 = ws.next().unwrap_or("0").parse().unwrap_or(0);
    println!("x={}, y={}", x, y);

    return [x, y]
}

fn proc_input(field: &Field, player: Player) -> [i8; 2] {
    loop {
        let xy = prompt(player);
        let x = xy[0];
        let y = xy[1];
        if !Field::is_in_field(x - 1, y - 1) {
            println!("input failed");
            continue;
        }

        if field.is_able_to_place(player, x - 1, y - 1) {
            println!("ok");
            return [x - 1, y - 1];
        } else {
            println!("cannot place there. input another place");
        }
    }
}
