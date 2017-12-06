use std::cmp;

fn get_shell(index: i32) -> i32 {
    return ((((index + 1) as f32).sqrt() - 1.0) / 2.0).ceil() as i32;
}

fn get_shell_start_index(shell: i32) -> i32 {
    if shell == 0 {
        return 0;
    }
    return 4 * (shell - 1) * shell + 1;
}

fn coord_from_index(index: i32) -> (i32, i32) {
    let shell = get_shell(index);
    let mut remainder = index - get_shell_start_index(shell);
    assert!(remainder >= 0);

    let side_length = shell * 2 + 1;

    let mut location = cmp::min(side_length - 1, 1);
    let mut side = 0;
    if remainder > 0 {
        loop {
            let step = cmp::min(side_length - location - 1, remainder);
            location = location + step;
            remainder = remainder - step;
            if remainder == 0 {
                break;
            }
            location = 0;
            side = side + 1;
        }
    }

    assert!(side < 4);
    assert!(side >= 0);

    let (start, direction) = match side {
        0 => ((shell, -shell), (0, 1)),
        1 => ((shell, shell), (-1, 0)),
        2 => ((-shell, shell), (0, -1)),
        3 => ((-shell, -shell), (1, 0)),
        _ => panic!(),
    };
    return (
        start.0 + location * direction.0,
        start.1 + location * direction.1,
    );
}

fn index_from_coord(coord: (i32, i32)) -> i32 {
    if coord == (0, 0) {
        return 0;
    }

    let shell = cmp::max(coord.0.abs(), coord.1.abs());
    let side = match coord {
        (_, y) if y == -shell => 3,
        (x, _) if x == -shell => 2,
        (_, y) if y == shell => 1,
        (x, _) if x == shell => 0,
        _ => panic!()
    };

    let mut index = get_shell_start_index(shell);
    if side > 0 {
        index += 2 * shell - 1;
    } else {
        index += coord.1 + shell - 1;
        return index;
    }

    if side > 1 {
        index += 2 * shell;
    } else {
        index += shell - coord.0;
        return index;
    }

    if side > 2 {
        index += 2 * shell;
    } else {
        index += shell - coord.1;
        return index;
    }

    index += coord.0 + shell;

    return index;
}

fn part2(target_value: i32) -> i32 {
    let mut v = vec![1];
    loop {
        let index = v.len() as i32;

        let coord = coord_from_index(index);
        let mut value: i32 = 0;
        for x_off in -1..2 {
            for y_off in -1..2 {
                let offset_coord = (coord.0 + x_off, coord.1 + y_off);
                let adjacent_index = index_from_coord(offset_coord);
                if adjacent_index < v.len() as i32 {
                    value += v[adjacent_index as usize];
                }
            }
        }
        if value > target_value {
            return value;
        }
        v.push(value);
    }
}

fn main() {
    let input = 325489;
    let index = input - 1;
    let coordinate = coord_from_index(index);
    println!("Part 1: {0}", coordinate.0.abs() + coordinate.1.abs());
    println!("Part 2: {0}", part2(input));
}
