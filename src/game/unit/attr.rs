mod offense;
mod defense;
mod about_self;

use num_rational::Ratio;

use crate::game::unit::bound::BoundPart;

use super::Unit;

impl Unit {
    // Basic
    pub fn str_adj(&self) -> i32 {
        (self.str_max - self.inj / self.inj_coefficient).max(0)
    }

    pub fn dex_adj(&self) -> i32 {
        (self.dex_max - self.inj / self.inj_coefficient).max(0)
    }

    pub fn agi_adj(&self) -> i32 {
        (self.agi_max - self.inj / self.inj_coefficient).max(0)
    }

    pub fn str(&self) -> i32 {
        if !self.is_able() {return 0;}
        self.str_adj()
    }

    pub fn dex(&self) -> i32 {
        if !self.is_able() {return 0;}
        self.dex_adj()
    }

    pub fn agi(&self) -> i32 {
        if !self.is_able() {return 0;}
        self.agi_adj()
    }

    fn walk_coefficient(&self) -> Ratio<i32> {
        // 步行系数
        // 倒地为0
        // 渐进
        // 脚腕  小腿  大腿  自由
        // 25%   50%  75%   100%
        if self.fall {
            Ratio::new(0, 4)
        } else if self.is_bound(&BoundPart::Ankle) {
            Ratio::new(1, 4)
        } else if self.is_bound(&BoundPart::Calve) {
            Ratio::new(2, 4)
        } else if self.is_bound(&BoundPart::Thigh) {
            Ratio::new(3, 4)
        } else {
            Ratio::new(4, 4)
        }
    }

    fn jump_coefficient(&self) -> Ratio<i32> {
        // 跳跃系数
        // 和双腿并拢跳跃有关的系数，倒地时与跳起站立有关
        // 如果反弓，则为0
        // 否则，基础值为50%，如上肢受限则为25%，倒地时额外减半
        if self.is_bound_bow() {
            return Ratio::new(0, 1);
        }
        let mut r = Ratio::new(1, 2);
        if !self.free_upper() {
            r /= 2;
        }
        if self.fall {
            r /= 2;
        }
        r
    }

    fn freedom_upper_coefficient(&self) -> Ratio<i32> {
        // 上肢自由度：
        // 被驷马或者悬挂则为0
        // 手腕和五花独立除2

        if self.is_bound(&BoundPart::Joint) || self.is_bound(&BoundPart::Hang) {
            return Ratio::new(0, 1)
        } 
        let mut r = Ratio::new(1, 1);
        
        if self.is_bound(&BoundPart::Wrist) {
            r /= 2;
        }
        if self.is_bound(&BoundPart::Neck) {
            r /= 2;
        }
        r
    }

    fn freedom_lower_coefficient(&self) -> Ratio<i32> {
        // 自由度下肢
        // 渐进
        // 反弓  脚腕or小腿  大腿  自由
        // 0%   50%         75%   100%
        if self.is_bound_bow() {
            return Ratio::new(0, 1);
        } else if self.is_bound(&BoundPart::Ankle) || self.is_bound(&BoundPart::Calve) {
            return Ratio::new(1, 2);
        } else if self.is_bound(&BoundPart::Thigh) {
            return Ratio::new(3, 4);
        } else {
            return Ratio::new(1, 1);
        }
    }

    fn freedom_hand_coefficient(&self) -> Ratio<i32> {
        // 手的自由度
        // 手腕被绑则为0
        // 胳膊被绑就减半，两层减至1/4
        if self.is_bound(&BoundPart::Wrist) {
            return Ratio::new(0, 1)
        } 
        let mut r = Ratio::new(1, 1);
        
        if self.is_bound(&BoundPart::Neck) {
            r /= 2;
        }
        if self.is_bound(&BoundPart::Arm) {
            r /= 2;
        }

        r
    }
    
}