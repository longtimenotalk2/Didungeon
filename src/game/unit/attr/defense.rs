use num_rational::Ratio;

use crate::game::unit::Unit;

impl Unit {
    pub fn evd(&self) -> i32 {
        // 闪避率
        // 从跑和跳两种模式中选一个最大的
        // 跑的模式：敏捷 * 步行系数
        // 跳的模式：敏捷 * 跳跃系数

        let coef = self.walk_coefficient().max(self.jump_coefficient());
        let r = Ratio::from_integer(self.agi()) * coef;
        r.ceil().to_integer()
    }

    pub fn evd_back(&self) -> i32 {
        // 被刺闪避率
        // 正常闪避率的一半
        let r = Ratio::from_integer(self.evd()) / 2;
        r.ceil().to_integer()
    }

    pub fn def_gym(&self) -> i32 {
        // 防御体术攻击
        // 上肢受限则为0
        match self.free_upper() {
            true => self.str(),
            false => 0,
        }
    }

    pub fn def_gym_back(&self) -> i32 {
        // 被刺防御体术攻击
        // 正常防御的一半
        let r = Ratio::from_integer(self.def_gym()) / 2;
        r.ceil().to_integer()
    }

    pub fn struggle_force(&self) -> i32 {
        // 反压制力
        // 上肢下肢自由度取平均

        let coef = (self.freedom_upper_coefficient() + self.freedom_lower_coefficient()) / 2;
        let r = Ratio::from_integer(self.str()) * coef;
        r.ceil().to_integer()
    }

    pub fn anti_tie_upper(&self) -> i32 {
        // 上肢反抗被捆绑力
        let r = Ratio::from_integer(self.str()) * self.freedom_upper_coefficient();
        r.ceil().to_integer()
    }

    pub fn anti_tie_lower(&self) -> i32 {
        // 下肢反抗被捆绑力
        let r = Ratio::from_integer(self.str()) * self.freedom_lower_coefficient();
        r.ceil().to_integer()
    }
}