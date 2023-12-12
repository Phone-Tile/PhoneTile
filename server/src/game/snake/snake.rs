use crate::network::player;
use std::collections::VecDeque;
type Grid = &mut Vec<&mut Vec<u16>>;

const max_players:u16 = 16;
const grid_offset:u32 = max_players+1;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Position{
    x:u32,
    y:u32,
}
pub struct Snake{
    player_id:u16,
    body:&mut VecDeque<Position,Direction>,
}

pub fn snake(players:Vec<player::Player>) {
    let grid = make_grid(players);
    n_players = len(players);
    let snakes : &mut Vec<Snake>;
    for i in 1..=n{
        let init_posi:Position; //TODO: définir posisition initiale en fonction du joueur (en gros chacun somewhat au centre de son écran)
        snakes.append((i,VecDeque::from([(init_posi,Up),(init_posi,Up),(init_posi,Up)])));
        let (xi,yi) = (init_posi.x,init_posi.y);
        grid[xi][yi] = i + 2 * grid_offset;
    }
}

fn make_grid(players:Vec)-> Grid {
    //TODO: Récupérer (somehow) les tailles d'écran
    //(0,0) will be the top left corner, x increases going right, y increases going down
    let total_length:u32;
    let total_width:u32;

    let square_side:u32; //TODO: A définir

    let controller_size:(u32,u32); //TODO: A définir
    let controller_length;controller_width = controller_size;

    let total_controller_length;total_controller_width; //TODO: Démerdez-vous je sais pas comment les appareils sont agencés

    let square_length = (total_length - total_controller_length)/square_side;
    let square_width = (total_width - total_controller_width)/square_side;
    let grid: Grid = &mut Vec::from([&mut Vec::from([0;square_width]);square_length]);
    grid
}



impl Snake {
    fn move_snake(self,grid:Grid) {
        let body: VecDeque<Position, Direction> = self.body;
        let new_head = next_head(body.back(),grid);
        let tail = body.pop_front();
        let (tail_pos,_) = tail;
            if grid[tail_pos.x][tail_pos.y] < grid_offset{
                grid[tail_pos.x][tail_pos.y] = 0
            }
            else {
                grid[tail_pos.x][tail_pos.y] -= grid_offset;
            }
        if  !is_occupied(new_head,grid) {
            body.push_back(new_head);
            let (head_pos,_) = new_head;
            grid[head_pos.x][head_pos.y] = self.player_id;
        }
        else {
            //TODO: mort
        }
    }

    fn grow(self,amount:usize,grid:Grid) {
        let (pos,_):(Position, Direction) = self.body.front();
        for i in 0..amount{
            self.body.push_front(self.body.front());
            grid[pos.x][pos.y] += grid_offset;
        }
    }
}

fn is_occupied(square:Position,grid:Grid)-> bool {
    let (x,y) = (square.x,square.y);
    grid[x][y] != 0
}

fn next_head(head:(Position, Direction),grid:Grid) -> (Position,Direction) {
    let (length,width) = (len(grid),len(grid[0]));
    let (pos,dir) = head;
    let (x,y) = (pos.x,pos.y);
    let (newx,newy);
    if dir == Up {
        newx = x;
        newy = (y-1) % width;
    }
    else if dir == Down {
        newx = x;
        newy = (y+1) % width;
    }
    else if dir == Right {
        newx = (x+1) % length;
        newy = y;
    }
    else {
        newx = (x-1) % length;
        newy = y;
    }
    let newpos:Position = (newx,newy);
    let new_head:(Position,Direction) = (newpos,dir);
    new_head
}
