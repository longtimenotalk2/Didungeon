use crate::game::unit::Unit;

impl Unit {
    pub fn acc_melee_hand(&self) -> i32 {
        // 手部近战技能相关精准
        // 灵巧 * 移动衰减 * 手部自由度衰减
        self.freedom_hand_coefficient(self.walk_coefficient(self.dex()))
    }

    pub fn push(&self) -> i32 {
        // 推力，用于计算推倒时使用
        // 力量 * 移动衰减 * 手部自由度衰减
        self.freedom_hand_coefficient(self.walk_coefficient(self.str()))
    }

    pub fn hold(&self) -> i32 {
        // 压倒对手时，的下压力
        // 倒地为0
        // 保底有50%的力量
        // 如果腿部完全自由，则加25%
        // 如果胳膊完全自由，再加25%
        if self.fall {return 0;}
        let mut r = 4;
        if self.bound_ankle || self.bound_calve || self.bound_thigh {r -= 1;}
        if self.bound_wrist || self.bound_neck {r -= 1;}
        self.str() * r / 4
    }

    pub fn tie_power(&self) -> i32 {
        // 捆绑时的力量
        // 力量 * 手部自由度衰减
        self.freedom_hand_coefficient(self.str())
    }
}