use std::io::{stdout, Stdout, Write, Read};
use termion::raw::{IntoRawMode, RawTerminal};

const WIDTH: u16 = 150;
const HEIGHT: u16 = 40;

#[derive(Clone)]
enum Object {
    None,
    Wall,
    DZone,
    Block,
}

enum State {
    Play,
    Over,
    Finish,
}

enum Direction {
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
}

struct Ball {
    x: u16,
    y: u16,
    dir: Direction,
}

fn main() {
    let mut stdin = termion::async_stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (mut field, mut blocks) = init_generate();
    let mut paddle_coord = (WIDTH / 2, HEIGHT - 2);
    let mut ball = Ball {
        x: paddle_coord.0,
        y: paddle_coord.1 - 3,
        dir: Direction::UpLeft,
    };

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    draw(&mut field, &mut stdout, &mut paddle_coord, &mut ball);
    let mut result = State::Play;

    loop {
        let mut input = [0];

        if stdin.read(&mut input).is_ok() {
            match input[0] {
                b'q' => { // optimize to use result
                    write!(stdout, "{}", termion::clear::All).expect("io error!");
                    stdout.flush().unwrap();
                    println!("Game Over");
                    break
                },
                b'd' => {
                    if paddle_coord.0 != 2 {
                        paddle_coord.0 += 5;
                    }
                },
                b'a' => {
                    if paddle_coord.0 != WIDTH - 2 {
                        paddle_coord.0 -= 5;
                    }
                },
                _ => {}
            }
            stdout.flush().unwrap();
        }
        
        result = progress(&mut field, &mut paddle_coord, &mut ball, &mut blocks);
        draw(&mut field, &mut stdout, &mut paddle_coord, &mut ball);

        match result {
            State::Finish => {
                write!(stdout, "{}", termion::clear::All).expect("io error!");
                stdout.flush().unwrap();
                println!("Well Done, You Finished!");
                break
            },
            State::Over => {
                write!(stdout, "{}", termion::clear::All).expect("io error!");
                stdout.flush().unwrap();
                println!("Game Over");
                break
            },
            _ => {}
        }

        std::thread::sleep(std::time::Duration::from_millis(60));
        write!(stdout, "{}", termion::clear::All).expect("io error!");
        stdout.flush().unwrap();
    }

    
}

fn progress(field: &mut Vec<Vec<Object>>, paddle_coord: &mut (u16, u16), ball: &mut Ball, blocks: &mut u32) -> State {
    let new_ball_y: u16;
    let new_ball_x: u16;

    match ball.dir {
        Direction::DownLeft => {
            new_ball_x = ball.x - 1;
            new_ball_y = ball.y + 1;
        }
        Direction::DownRight => {
            new_ball_x = ball.x + 1;
            new_ball_y = ball.y + 1;
        }
        Direction::UpLeft => {
            new_ball_x = ball.x - 1;
            new_ball_y = ball.y - 1;
        }
        Direction::UpRight => {
            new_ball_x = ball.x + 1;
            new_ball_y = ball.y - 1;
        }
    }

    match field[new_ball_x as usize][new_ball_y as usize] {
        Object::Wall => {
            *ball = Ball {
                x: ball.x,
                y: ball.y,
                dir: match ball.dir {
                    Direction::DownLeft => Direction::DownRight,
                    Direction::DownRight => Direction::DownLeft,
                    Direction::UpLeft => Direction::UpRight,
                    Direction::UpRight => Direction::UpLeft,
                },
            };
            progress(field, paddle_coord, ball, blocks);
        }
        Object::Block => {
            *ball = Ball {
                x: ball.x,
                y: ball.y,
                dir: match ball.dir {
                    Direction::DownLeft => Direction::UpLeft,
                    Direction::DownRight => Direction::UpRight,
                    Direction::UpLeft => Direction::DownLeft,
                    Direction::UpRight => Direction::DownRight,
                },
            };
            field[new_ball_x as usize][new_ball_y as usize] = Object::None;
            *blocks -= 1;

            progress(field, paddle_coord, ball, blocks);
        }
        Object::DZone => return State::Over,
        Object::None => {
            if (new_ball_x == paddle_coord.0 - 2
                || new_ball_x == paddle_coord.0 - 1
                || new_ball_x == paddle_coord.0
                || new_ball_x == paddle_coord.0 + 1
                || new_ball_x == paddle_coord.0 + 2
                || new_ball_x == paddle_coord.0 + 3
                || new_ball_x == paddle_coord.0 - 3
                || new_ball_x == paddle_coord.0 + 4
                || new_ball_x == paddle_coord.0 - 4
                || new_ball_x == paddle_coord.0 - 5
                || new_ball_x == paddle_coord.0 + 5
            )
                && new_ball_y == paddle_coord.1
            {
                *ball = Ball {
                    x: ball.x,
                    y: ball.y,
                    dir: match ball.dir {
                        Direction::DownLeft => Direction::UpLeft,
                        Direction::DownRight => Direction::UpRight,
                        _ => panic!("HOW THE FUDGE..."),
                    },
                };
            } else {
                ball.x = new_ball_x;
                ball.y = new_ball_y;
            }
        }
    }

    State::Play
}

fn draw(
    field: &mut Vec<Vec<Object>>,
    stdout: &mut RawTerminal<Stdout>,
    paddle_coord: &mut (u16, u16),
    ball: &mut Ball,
) {
    for index_x in 0..field.len() {
        for index_y in 0..field[index_x].len() {
            match field[index_x][index_y] {
                Object::Wall => write!(
                    stdout,
                    "{}|",
                    termion::cursor::Goto(index_x as u16 + 1, index_y as u16 + 1)
                )
                .unwrap(),
                Object::Block => write!(
                    stdout,
                    "{}V",
                    termion::cursor::Goto(index_x as u16 + 1, index_y as u16 + 1)
                )
                .unwrap(),
                Object::DZone => write!(
                    stdout,
                    "{}#",
                    termion::cursor::Goto(index_x as u16 + 1, index_y as u16 + 1)
                )
                .unwrap(),
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    write!(stdout, "{}O", termion::cursor::Goto(ball.x, ball.y)).unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 + 1, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 - 1, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 - 2, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 + 2, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 - 3, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 + 3, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 + 4, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 - 4, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 - 5, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
    write!(
        stdout,
        "{}_",
        termion::cursor::Goto(paddle_coord.0 + 5, paddle_coord.1)
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn init_generate() -> (Vec<Vec<Object>>, u32) {
    let mut field = vec![vec![Object::None; HEIGHT as usize + 1]; WIDTH as usize + 1];
    let mut blocks = 0;

    for x_index in 0..field.len() {
        field[x_index][HEIGHT as usize] = Object::Wall;
        field[x_index][0] = Object::Wall;
    }

    for y_index in 0..field[0].len() {
        field[0][y_index] = Object::Wall;
        field[WIDTH as usize][y_index] = Object::Wall;
    }

    for x_index in 1..field.len() - 1 {
        for y_index in 2..8 {
            field[x_index][y_index] = Object::Block;
            blocks += 1;
        }
    }

    for x_index in 1..field.len() - 1 {
        field[x_index][HEIGHT as usize] = Object::DZone;
    }

    (field, blocks)
}
