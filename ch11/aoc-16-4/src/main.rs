#[derive(Debug)]
struct Room<'a> {
    name: &'a str,
    id: u64,
    chcksum: &'a str,
}

impl<'a> Room<'a> {
    fn parse(room: &'a str) -> Option<Self> {
        let room = room.trim();
        let (room, tail) = room.split_once('[')?;
        let (chcksum, _) = tail.split_once(']')?;

        let (name, id) = room.rsplit_once('-')?;
        let id = id.parse().ok()?;

        Some(Self { name, id, chcksum })
    }

    fn is_valid(&self) -> bool {
        let count_byte = |b| self.name.bytes().filter(|c| *c == b).count();

        self.chcksum.as_bytes().windows(2).all(|w| {
            let (l, r) = (w[0], w[1]);
            let lcnt = count_byte(l);
            let rcnt = count_byte(r);

            lcnt > rcnt || (lcnt == rcnt && l < r)
        })
    }
}

fn parse(input: &str) -> Vec<Room> {
    input.lines().filter_map(Room::parse).collect()
}

fn solve(rooms: &[Room]) -> u64 {
    rooms.iter().filter(|r| r.is_valid()).map(|r| r.id).sum()
}

fn main() {
    let input = include_str!("../input");
    let rooms = dbg!(parse(input));
    let result = solve(&rooms);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../test");
        let rooms = parse(input);
        let result = super::solve(&rooms);

        assert_eq!(1514, result);
    }
}
