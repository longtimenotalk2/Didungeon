use super::{board::{Board, turn::ChooseSkill}, unit::Id};

pub struct AI {

}

impl AI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyse_skill(&self, board : &Board, id : Id, chooses : &[ChooseSkill]) -> ChooseSkill {
        // 先按顺序执行，只要不是move
        if chooses.len() == 1 {
            chooses[0].clone()
        }else{
            if let ChooseSkill::Move {..} = chooses.get(1).unwrap() {
                let mut min_condition : Option<(usize, i32)> = None;
                for (i, skl) in chooses.iter().enumerate() {
                    if let ChooseSkill::Move {pos, dir : _} = skl{
                        if let Some(dist) = board.find_dist_of_no_defeated_enemy(id, pos) {
                            match min_condition {
                                Some((_, disto)) => {
                                    if dist < disto {
                                        min_condition = Some((i, dist));
                                    }
                                },
                                None => min_condition = Some((i, dist)),
                            }
                        }

                    }
                }
                if let Some((i, _)) = min_condition {
                    chooses[i].clone()
                }else{
                    chooses[0].clone()
                }
            }else{
                chooses[1].clone()
            }
        }
    }
}