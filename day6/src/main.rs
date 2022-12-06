pub fn is_valid_marker(marker: &[u8]) -> bool {
    for i in 1..marker.len() {
        if marker[i..].contains(&marker[i - 1]) {
            return false;
        }
    }

    true
}

fn main() {
    let input = include_bytes!("../input");

    let pos1 = input
        .windows(4)
        .position(is_valid_marker)
        .map(|pos| pos + 4);

    println!("{:?}", pos1);

    let pos2 = input
        .windows(14)
        .position(is_valid_marker)
        .map(|pos| pos + 14);

    println!("{:?}", pos2);
}
