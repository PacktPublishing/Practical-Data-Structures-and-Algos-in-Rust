#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Dir(u8);

impl Dir {
    const N: Dir = Dir(0);

    fn left(self) -> Dir {
        let Dir(dir) = self;
        Dir((dir + 1) % 4)
    }

    fn rigth(self) -> Dir {
        let Dir(dir) = self;
        Dir((dir + 3) % 4)
    }

    fn turn(self, turn: Turn) -> Dir {
        match turn {
            Turn::Left => self.left(),
            Turn::Right => self.rigth(),
        }
    }

    fn delta(self) -> (i64, i64) {
        match self.0 {
            0 => (0, 1),
            1 => (-1, 0),
            2 => (0, -1),
            3 => (1, 0),
            _ => (0, 0),
        }
    }
}

struct Cmd {
    turn: Turn,
    steps: i64,
}

impl Cmd {
    fn parse(command: &str) -> Option<Self> {
        let (turn, steps) = command.trim().split_at(1);

        let turn = match turn {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => return None,
        };

        let steps = steps.parse().ok()?;

        Some(Self { turn, steps })
    }
}

struct State {
    dir: Dir,
    pos: (i64, i64),
}

fn main() {
    let input = std::io::stdin().lines().next();
    let input = input.and_then(|line| line.ok());
    let input = input.as_deref().unwrap_or("");

    let state = input.split(',').filter_map(Cmd::parse).fold(
        State {
            dir: Dir::N,
            pos: (0, 0),
        },
        |state, command| {
            let dir = state.dir.turn(command.turn);
            let (dx, dy) = dir.delta();
            let (x, y) = state.pos;
            let x = x + dx * command.steps;
            let y = y + dy * command.steps;

            State { dir, pos: (x, y) }
        },
    );

    let (x, y) = state.pos;
    let dist = x.abs() + y.abs();

    println!("{dist}");
}
