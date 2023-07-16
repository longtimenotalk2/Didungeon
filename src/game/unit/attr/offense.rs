use num_rational::Ratio;

use super::Unit;

impl Unit {
    pub fn acc_melee_hand(&self) -> i32 {
        // 手部近战技能相关精准
        // 灵巧 * 移动衰减 * 手部自由度衰减
        let r = Ratio::from_integer(self.dex()) * self.walk_coefficient() * self.freedom_hand_coefficient();
        r.ceil().to_integer()
    }

    pub fn hold_force(&self) -> i32 {
        // 压制力
        // 倒地为0
        // 如果腿部完全自由，占50%
        // 如果胳膊完全自由，占50%
        if self.fall {return 0;}
        let mut r = Ratio::new(0, 1);
        if self.free_upper() {
            r += Ratio::new(1, 2);
        }
        if self.free_lower() {
            r += Ratio::new(1, 2);
        }
        r.ceil().to_integer()
    }

    pub fn tie_power(&self) -> i32 {
        // 捆绑时的力量
        // 力量 * 手部自由度衰减
        let r = Ratio::from_integer(self.str()) * self.freedom_hand_coefficient();
        r.ceil().to_integer()
    }
}