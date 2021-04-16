pub fn get_line_from_pos(pos: usize) -> usize {
    return pos / 19;
}

pub fn is_on_axe(
    axe_increment_value: usize,
    move_pos: usize,
    axe_distance: usize,
    direction_sign: i16,
) -> bool {
    if (move_pos as isize
        + axe_increment_value as isize * axe_distance as isize * direction_sign as isize)
        < 0
        || (move_pos as isize
            + axe_increment_value as isize * axe_distance as isize * direction_sign as isize)
            > 360
    {
        return false;
    } else if axe_increment_value == 1 {
        if get_line_from_pos(
            (move_pos as isize
                + axe_increment_value as isize * axe_distance as isize * direction_sign as isize)
                as usize,
        ) != get_line_from_pos(move_pos)
        {
            return false;
        }
    } else {
        if get_line_from_pos(
            (move_pos as isize
                + axe_increment_value as isize * axe_distance as isize * direction_sign as isize)
                as usize,
        ) != (get_line_from_pos(move_pos) as isize
            + axe_distance as isize * direction_sign as isize) as usize
        {
            return false;
        }
    }
    return true;
}
