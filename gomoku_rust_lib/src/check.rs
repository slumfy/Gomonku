fn check_is_in_table(x: i32, y: i32, xsign: i32, ysign: i32, offset: i32) -> i32 {
    if x + offset * xsign > 18
        || x + offset * xsign < 0
        || y + offset * ysign > 18
        || y + offset * ysign < 0
    {
        return 1;
    }
    return 0;
}

pub fn check_win_position(mut map: &Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> i32 {
    let mut it = 0;
    let mut tmp = 0;
    tmp = check_win_routine(map, player, x, y, 1, 0);
    if tmp > it {
        it = tmp;
    }
    tmp = check_win_routine(map, player, x, y, 0, 1);
    if tmp > it {
        it = tmp;
    }
    tmp = check_win_routine(map, player, x, y, -1, 1);
    if tmp > it {
        it = tmp;
    }
    tmp = check_win_routine(map, player, x, y, 1, 1);
    if tmp > it {
        it = tmp;
    }
    return it;
}

pub fn check_wrong_position(mut map: &Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> i32 {
    if check_is_in_table(x, y, 0, 0, 0) == 1 || map[x as usize][y as usize] != 0 {
        return 1;
    }
    if check_three_position(map, player, x, y) != 0 {
        return 1;
    } else if check_wrong_routine(map, player, x, y, 1, 0) == 1 {
        return 1;
    } else if check_wrong_routine(map, player, x, y, 0, 1) == 1 {
        return 1;
    } else if check_wrong_routine(map, player, x, y, -1, 1) == 1 {
        return 1;
    } else if check_wrong_routine(map, player, x, y, 1, 1) == 1 {
        return 1;
    }
    return 0;
}

pub fn check_three_position(mut map: &Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> i32 {
    let mut three_count = 0;
    if check_three_routine(map, player, x, y, 1, 0) == 1 {
        three_count += 1;
    }
    if check_three_routine(map, player, x, y, 0, 1) == 1 {
        three_count += 1;
    }
    if check_three_routine(map, player, x, y, -1, 1) == 1 {
        three_count += 1;
    }
    if check_three_routine(map, player, x, y, 1, 1) == 1 {
        three_count += 1;
    }
    if three_count < 2 {
        return 0;
    }
    return 1;
}

pub fn check_eat_position(mut map: &Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> i32 {
	let mut eat_nb = 0;
    eat_nb += eat_position_routine(map, player, x, y, 0, 1);
    eat_nb += eat_position_routine(map, player, x, y, 0, -1);
    eat_nb += eat_position_routine(map, player, x, y, 1, 0);
    eat_nb += eat_position_routine(map, player, x, y, -1, 0);
    eat_nb += eat_position_routine(map, player, x, y, 1, 1);
    eat_nb += eat_position_routine(map, player, x, y, 1, -1);
    eat_nb += eat_position_routine(map, player, x, y, -1, 1);
    eat_nb += eat_position_routine(map, player, x, y, -1, -1);
	println!("manger {}", eat_nb);
	return eat_nb;
}

fn check_win_routine(
    mut map: &Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
    xsign: i32,
    ysign: i32,
) -> i32 {
    let mut it = 1;
    for n in 1..18 {
        if check_is_in_table(x, y, xsign, ysign, n) == 0
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    for n in 1..18 {
        if check_is_in_table(x, y, -xsign, -ysign, n) == 0
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    return it;
}

fn check_wrong_routine(
    mut map: &Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
    xsign: i32,
    ysign: i32,
) -> i32 {
    let mut it = 1;
    let mut uptrap = 0;
    let mut downtrap = 0;
    for n in 1..3 {
        if check_is_in_table(x, y, xsign, ysign, n) == 0
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] != player
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] != 0
        {
            uptrap = 1;
        } else if check_is_in_table(x, y, xsign, ysign, n) == 0
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    for n in 1..3 {
        if check_is_in_table(x, y, -xsign, -ysign, n) == 0
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] != player
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] != 0
        {
            downtrap = 1;
        } else if check_is_in_table(x, y, -xsign, -ysign, n) == 0
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    if it <= 2 && uptrap == 1 && downtrap == 1 {
        return 1;
    }
    return 0;
}

pub fn check_three_routine(
    mut map: &Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
    xsign: i32,
    ysign: i32,
) -> i32 {
    let mut it = 1;
    let mut uptrap = 0;
    let mut downtrap = 0;
    for n in 1..4 {
        if check_is_in_table(x, y, xsign, ysign, n) == 0
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] == 0
        {
            uptrap = 1;
        } else if check_is_in_table(x, y, xsign, ysign, n) == 0
            && map[(x + n * xsign) as usize][(y + n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    for n in 1..4 {
        if check_is_in_table(x, y, -xsign, -ysign, n) == 0
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] == 0
        {
            downtrap = 1;
        } else if check_is_in_table(x, y, -xsign, -ysign, n) == 0
            && map[(x - n * xsign) as usize][(y - n * ysign) as usize] == player
        {
            it += 1;
        } else {
            break;
        }
    }
    if it >= 3 && uptrap == 1 && downtrap == 1 {
        return 1;
    } else {
        return 0;
    }
}

pub fn eat_position_routine(
    mut map: &Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
    xsign: i32,
    ysign: i32,
) -> i32 {
    let mut poslist = Vec::new();
	let mut eat_nb = 0; 
    if check_is_in_table(x, y, xsign, ysign, 1) == 0
        && map[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != player
        && map[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != 0
    {
        if check_is_in_table(x, y, xsign, ysign, 2) == 0
            && map[(x + 2 * xsign) as usize][(y + 2 * ysign) as usize] == player
        {
            poslist.push((x + 1 * xsign, y + 1 * ysign));
            eat_nb += eat_stone(map, poslist);
        } else if check_is_in_table(x, y, xsign, ysign, 2) == 0
            && map[(x + 2 * xsign) as usize][(y + 2 * ysign) as usize] != 0
        {
            if check_is_in_table(x, y, xsign, ysign, 3) == 0
                && map[(x + 3 * xsign) as usize][(y + 3 * ysign) as usize] == player
            {
                poslist.push((x + 1 * xsign, y + 1 * ysign));
                poslist.push((x + 2 * xsign, y + 2 * ysign));
                eat_nb += eat_stone(map, poslist);
            }
        }
    }
	return eat_nb;
}

fn eat_stone(mut map: &Vec<Vec<i32>>, poslist: Vec<(i32, i32)>) -> i32 {
	let mut eat_nb = 0; 
    for (x, y) in poslist {
        map[x as usize][y as usize] = 0;
        eat_nb += 1;
    }
	return eat_nb;
}
