use super::{board::{Board, turn::ChooseSkill}, unit::Id, skill::Skill};


#[derive(Debug)]
struct Analyse {
    friend_catch_count : i32,
    self_unbound : i32,
    self_rescure : i32,
    self_catch : i32,
    self_attack : i32,
    team_dist_undefeated_enemy_count : i32,
}

impl Analyse {
    fn is_better_then(&self, other : &Self) -> bool {
        enum R {True, False, None}

        let judje = |a1 : i32, a2 : i32, more_is_better : bool| -> R {
            if a1 != a2 && (a1 > a2) == more_is_better {
                R::True
            }else if a1 != a2 && (a1 < a2) == more_is_better {
                R::False
            }else{
                R::None
            }
        };
        
        let mut match_seq  = vec![];
        // 队友的抓取个数，越多越好
        match_seq.push((self.friend_catch_count, other.friend_catch_count, true));
        // 自救
        match_seq.push((self.self_unbound, other.self_unbound, true));
        // 自己抓取个数，越多越好
        match_seq.push((self.self_catch, other.self_catch, true));
        // 我方解救个数，越多护额好
        match_seq.push((self.self_rescure, other.self_rescure, true));
        // 自己输出，越多越好
        match_seq.push((self.self_attack, other.self_attack, true));
        // 我方距离最近敌方非战败角色的总距离，越近越好
        match_seq.push((self.team_dist_undefeated_enemy_count, other.team_dist_undefeated_enemy_count, false));

        for (a1, a2, more_is_better) in match_seq {
            match judje(a1, a2, more_is_better) {
                R::True => return true,
                R::False => return false,
                R::None => continue,
            }
        }
        false
    }

    fn get_from_board_and_skill(board : &Board, is_ally : bool, skill : Option<&Skill>) -> Self {
        let mut friend_catch_count = 0;
        let mut self_unbound = 0;
        let mut self_rescure = 0;
        let mut self_catch = 0;
        let mut self_attack = 0;
        let mut team_dist_undefeated_enemy_count = 0;
        for unit in board.get_all_unit() {
            if unit.get_ally() == is_ally {
                if unit.get_catch().is_some() {
                    friend_catch_count += 1;
                }
                if let Some(Skill::Unbound) = skill {
                    self_unbound += 1;
                }
                if let Some(Skill::Untie) = skill {
                    self_rescure += 1;
                }
                if let Some(Skill::Catch) = skill {
                    self_catch += 1;
                }
                if let Some(Skill::Punch) = skill {
                    self_attack += 1;
                }
                if let Some(dist) = board.find_dist_of_no_defeated_enemy(unit.get_id(), &unit.get_pos()) {
                    team_dist_undefeated_enemy_count += dist;
                }
            }
        }
        Self { 
            friend_catch_count, 
            self_unbound,
            self_rescure,
            self_catch, 
            self_attack,
            team_dist_undefeated_enemy_count,
        }
    }
}

pub struct AI {

}

impl AI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyse_skill(&self, board : &Board, id : Id, chooses : &[ChooseSkill]) -> Option<ChooseSkill> {
        let is_ally = board.get_unit(id).get_ally();
        let mut select : Option<(usize, Analyse)> = None;
        for (i, skl) in chooses.iter().enumerate() {
            let analyse = match skl {
                ChooseSkill::Pass => Analyse::get_from_board_and_skill(board, is_ally, None),
                ChooseSkill::Skill { skill, it, dir } => Analyse::get_from_board_and_skill(&skill.analyse(board, id, *it, dir), is_ally, Some(skill)),
                ChooseSkill::Move { pos, dir } => {
                    let mut board = board.clone();
                    board.actor_move_to(id, *pos, dir.clone());
                    Analyse::get_from_board_and_skill(&board, is_ally, None)
                }
            };
            // dbg!(&skl);
            // dbg!(&analyse);
            match &select {
                Some((_, anl)) => {
                    if analyse.is_better_then(anl) {
                        select = Some((i, analyse));
                    }
                },
                None => select = Some((i, analyse)),
            }
            // dbg!(&select);
        }

        select.map(|(i, _)| chooses[i].clone())
    }
}