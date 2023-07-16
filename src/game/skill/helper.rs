use crate::wyrand::Dice;

use super::{super::unit::{Dir, Unit}, Skill};

use colorful::Color;
use colorful::Colorful;

pub fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

pub fn to_dmg(dmg_o : i32, min_dmg_set : i32) -> i32 {
    dmg_o.max(min_dmg_set)
}

pub fn show_announce(
    actor : &Unit, 
    target : &Unit, 
    dir : &Dir, 
    skill : &Skill,
) {
    actor.show_identity();
    print!(" {}{} ", skill.name(), dir.notice());
    target.show_identity()
}

pub fn hit_check(hit : i32, dice : &mut Dice) -> (bool, Option<i32>) {
    if hit == 100 {
        (true, None)
    }else if hit == 0 {
        (false, None)
    }else{
        let hit_dice = dice.d(100);
        if hit >= hit_dice {
            (true, Some(hit_dice))
        }else{
            (false, Some(hit_dice))
        }
    }
}

pub fn show_hit(
    hit : i32, 
    is_hit : bool, 
    hit_dice : Option<i32>,
    name : &str,
    name_hit : &str,
    name_miss : &str,
) {
    print!("{name} : {hit}%");
    if let Some(hit_dice) = hit_dice {
        print!("(🎲 = {})", 
            match is_hit {
                true => hit_dice.to_string().color(Color::Green),
                false => hit_dice.to_string().color(Color::Red),
            }
        );
    }
    match is_hit {
        true => print!("{}", name_hit.to_string().color(Color::Green)),
        false => print!("{}", name_miss.to_string().color(Color::Red)),
    }
}

pub fn show_dmg(dmg : i32, inj_old : i32, inj_new : i32) {
    print!("造成伤害 : {dmg} (负伤 : {inj_old} -> {})", inj_new.to_string().color(Color::Red))
}