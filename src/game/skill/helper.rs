use crate::wyrand::Dice;

use super::{super::unit::{Dir, Unit}, Skill};

use colorful::Color;
use colorful::Colorful;
use std::fmt::Write;

pub fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

pub fn to_dmg(dmg_o : i32, min_dmg_set : i32) -> i32 {
    dmg_o.max(min_dmg_set)
}

pub fn write_announce (
    target : &Unit, 
    dir : &Dir, 
    skill : &Skill,
) -> String  {
    let mut s = String::new();
    write!(&mut s, "{}{} {}", skill.name(), dir.notice(), target.identity()).unwrap();
    s
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

pub fn write_hit(
    s : &mut String,
    hit : i32, 
    is_hit : bool, 
    hit_dice : Option<i32>,
    name : &str,
    name_hit : &str,
    name_miss : &str,
) {
    write!(s, "{name} : {hit}%").unwrap();
    if let Some(hit_dice) = hit_dice {
        write!(s, " (🎲 = {})", 
            match is_hit {
                true => hit_dice.to_string().color(Color::Green),
                false => hit_dice.to_string().color(Color::Red),
            }
        ).unwrap();
    }
    match is_hit {
        true => write!(s, " -> {}\n", name_hit.to_string().color(Color::Green)).unwrap(),
        false => write!(s, " -> {}\n", name_miss.to_string().color(Color::Red)).unwrap(),
    }
    println!()
}

pub fn write_hit_small(
    s : &mut String,
    hit : i32, 
    is_hit : bool, 
    hit_dice : i32,
) {
    let hit_idy = hit.to_string().color(Color::Yellow);
    let dice_idy = match is_hit {
        true => hit_dice.to_string().color(Color::Green),
        false => hit_dice.to_string().color(Color::Red),
    };
    write!(s, "({hit_idy}%成功率 -> 🎲 : {dice_idy})").unwrap()
}

pub fn write_dmg(s : &mut String, dmg : i32, inj_old : i32, inj_new : i32) {
    writeln!(s, "造成伤害 : {dmg} (负伤 : {inj_old} -> {})", inj_new.to_string().color(Color::Red)).unwrap();
}