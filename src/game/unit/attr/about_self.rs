use num_rational::Ratio;

use crate::game::unit::{Unit, bound::BoundPart};

impl Unit {
    pub fn spd(&self) -> i32 {
        // 上下肢自由度求和
        let coef = (self.freedom_upper_coefficient() + self.freedom_lower_coefficient()) / 2;
        let r = Ratio::from_integer(self.agi()) * coef;
        r.ceil().to_integer()
    }

    pub fn can_stand(&self) -> bool {
        // 必须上肢下肢有一边完全自由才行，可以100%起身
        // 否则只能杂技起身，暂时不考虑
        self.free_upper() || self.free_lower()
    }

    pub fn move_range(&self) -> i32 {
        // 步行距离
        // 从跑和跳两种模式中选一个最大的
        // 跑的模式：敏捷 * 步行系数
        // 跳的模式：敏捷 * 跳跃系数
        // 最终 / 5

        let coef = self.walk_coefficient().max(self.jump_coefficient());
        let r = Ratio::from_integer(self.agi()) * coef;
        (r / 5).floor().to_integer()
    }

    pub fn hand_str(&self) -> i32 {
        // 手部力量相关
        // 力量 * 手部自由度系数
        let r = Ratio::from_integer(self.str()) * self.freedom_hand_coefficient();
        r.ceil().to_integer()
    }

    pub fn hand_dex(&self) -> i32 {
        // 手部操作相关灵巧
        // 灵巧 * 手部自由度系数
        let r = Ratio::from_integer(self.dex()) * self.freedom_hand_coefficient();
        r.ceil().to_integer()
    }

    pub fn unbound_force_upper(&self) -> i32 {
        // 上肢蛮力挣脱力
        // 独立占比
        // 五花  胳膊  悬挂
        // 50%   25%  25%
        // 如果joint，最终再除2，如果wrist，最终再除2

        let mut r = Ratio::new(1, 1);
        if self.is_bound(&BoundPart::Neck) {
            r -= Ratio::new(1, 2);
        }
        if self.is_bound(&BoundPart::Arm) {
            r -= Ratio::new(1, 4);
        }
        if self.is_bound(&BoundPart::Hang) {
            r -= Ratio::new(1, 4);
        }
        if self.is_bound(&BoundPart::Joint) {
            r /= 2;
        }
        if self.is_bound(&BoundPart::Wrist) {
            r /= 2;
        }

        let r = Ratio::from_integer(self.str()) * r;
        r.ceil().to_integer()
    }

    pub fn unbound_force_lower(&self) -> i32 {
        // 下肢蛮力挣脱力
        // 独立占比
        // 大腿  小腿  脚腕
        // 25%   25%   50%
        // 如果反弓，额外/2

        if self.free_lower() {
            return self.str();
        }
        
        let mut r = Ratio::new(1, 1);
        if self.is_bound(&BoundPart::Thigh) {
            r -= Ratio::new(1, 4);
        }
        if self.is_bound(&BoundPart::Calve) {
            r -= Ratio::new(1, 4);
        }
        if self.is_bound(&BoundPart::Ankle) {
            r -= Ratio::new(1, 2);
        }
        if self.is_bound_bow() {
            r /= 2
        }

        let r = Ratio::from_integer(self.str()) * r;
        r.ceil().to_integer()
    }

}