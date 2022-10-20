use bevy::prelude::*;
use crate::constants::*;

pub fn is_equal(transform_one: &Transform, transform_two: &Transform) -> bool {
    transform_one.translation.x == transform_two.translation.x
        && transform_one.translation.y == transform_two.translation.y
}

pub fn is_equal_approximate(transform_one: &Transform, transform_two: &Transform) -> bool {
    ((transform_two.translation.x - CELL_OFFSET)..(transform_two.translation.x + CELL_OFFSET)).contains(&transform_one.translation.x )
        && ((transform_two.translation.y - CELL_OFFSET)..(transform_two.translation.y + CELL_OFFSET)).contains(&transform_one.translation.y )
}