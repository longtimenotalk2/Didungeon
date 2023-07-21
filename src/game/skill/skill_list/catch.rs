use crate::game::{unit::{Unit, Id, Dir}, skill::{Skillize, Skill, helper}, board::Board};
pub struct Catch {

}

impl Catch {
    pub fn new() -> Self {
        Self {}
    }

    pub fn can(&self, actor : &Unit, target : &Unit) -> bool {
        actor.hold_force() > target.struggle_force() && actor.acc_melee_hand() > target.evd()
    }
}

impl Skillize for Catch {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        let mut list: Vec<_> = vec!();
        for (it, dir) in board.find_adjs(id) {
            if self.can(board.get_unit(id), board.get_unit(it)) {
                list.push((it, dir));
            }
        }
        list
    }

    fn exe(&self, s : &mut String, board : &mut Board, id : Id, it : Id, dir : &Dir) {
        let target = board.get_unit(it);

        // 宣言
        *s += &helper::write_announce(target, &dir, &Skill::Catch);

        // 结算
        board.get_unit_mut(id).catch_with(it, dir);
        board.get_unit_mut(it).catched_with(id, dir);
        board.get_unit_mut(it).take_fall();

        board.set_to_end(id);
    }

    fn choice_show(&self, board : &Board, _id : Id, it : Id, dir : &Dir) -> String {
        let mut st = String::new();

        let target = board.get_unit(it);
        st += &helper::write_announce( target, dir, &Skill::Catch);

        if !target.is_fall() {
            st += " (扑倒)";
        }

        st += " -> 捆绑";
        st
    }
}
