use super::{board::{Board, turn::ChooseSkill}, unit::Id};

/*
逻辑：
不破坏队友的擒拿状态
移动：优先移动至距离非战败敌人更近的位置
*/

#[derive(Debug)]
struct Analyse {
    friend_catch_count : i32,
    friend_dist_undefeated_enemy_count : i32,
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
        match_seq.push((self.friend_catch_count, other.friend_catch_count, true));
        match_seq.push((self.friend_dist_undefeated_enemy_count, other.friend_dist_undefeated_enemy_count, false));

        for (a1, a2, more_is_better) in match_seq {
            match judje(a1, a2, more_is_better) {
                R::True => return true,
                R::False => return false,
                R::None => continue,
            }
        }
        false
    }

    fn get_from_board(board : &Board, is_ally : bool) -> Self {
        let mut friend_catch_count = 0;
        let mut friend_dist_undefeated_enemy_count = 0;
        for unit in board.get_all_unit() {
            if unit.get_ally() == is_ally {
                if unit.get_catch().is_some() {
                    friend_catch_count += 1;
                }
                if let Some(dist) = board.find_dist_of_no_defeated_enemy(unit.get_id(), &unit.get_pos()) {
                    friend_dist_undefeated_enemy_count += dist;
                }
            }
        }
        Self { friend_catch_count, friend_dist_undefeated_enemy_count }
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
                ChooseSkill::Pass => Analyse::get_from_board(board, is_ally),
                ChooseSkill::Skill { skill, it, dir } => Analyse::get_from_board(&skill.analyse(board, id, *it, dir), is_ally),
                ChooseSkill::Move { pos, dir } => {
                    let mut board = board.clone();
                    board.actor_move_to(id, *pos, dir.clone());
                    Analyse::get_from_board(&board, is_ally)
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