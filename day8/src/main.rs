use std::collections::HashSet;

fn mark_visible(
    output: &mut HashSet<(usize, usize)>,
    input: &[&[u8]],
    mut x: usize,
    mut y: usize,
    step_x: isize,
    step_y: isize,
) {
    let mut prev = 0; // zero byte is smaller than '0'

    while y < input.len() && x < input[y].len() {
        if input[y][x] > prev {
            output.insert((x, y));
            prev = input[y][x];
        }

        x = (x as isize + step_x) as usize;
        y = (y as isize + step_y) as usize;
    }
}

fn view_distance(input: &[&[u8]], mut x: usize, mut y: usize, step_x: isize, step_y: isize) -> u32 {
    let height = input[y][x];
    let mut count = 0;

    loop {
        x = (x as isize + step_x) as usize;
        y = (y as isize + step_y) as usize;

        if y >= input.len() || x >= input[y].len() {
            break;
        }

        count += 1;

        if input[y][x] >= height {
            break;
        }
    }

    count
}

fn main() {
    let input = include_str!("../input");
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut visible = HashSet::new();

    for x in 0..input.len() {
        mark_visible(&mut visible, &input, x, 0, 0, 1);
        mark_visible(&mut visible, &input, x, input[0].len() - 1, 0, -1);
    }

    for y in 0..input[0].len() {
        mark_visible(&mut visible, &input, 0, y, 1, 0);
        mark_visible(&mut visible, &input, input.len() - 1, y, -1, 0);
    }

    println!("{:?}", visible.len());

    let mut min_score = 0;
    for x in 1..input.len() - 1 {
        for y in 1..input[x].len() - 1 {
            let score = view_distance(&input, x, y, 1, 0)
                * view_distance(&input, x, y, -1, 0)
                * view_distance(&input, x, y, 0, 1)
                * view_distance(&input, x, y, 0, -1);

            if score > min_score {
                min_score = score;
            }
        }
    }

    println!("{min_score}");
}
