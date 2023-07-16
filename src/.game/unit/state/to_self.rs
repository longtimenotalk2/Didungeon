use crate::game::unit::Unit;

impl Unit {
    pub fn spd(&self) -> i32 {
        // 上下肢自由度求和
        (self.freedom_lower_coefficient(self.agi()) + 
        self.freedom_upper_coefficient(self.agi()) ) / 2
    }

    pub fn can_stand(&self) -> bool {
        // 必须上肢下肢有一边完全自由才行，可以100%起身
        // 否则只能杂技起身，暂时不考虑
        let upper = self.bound_ankle || self.bound_calve || self.bound_thigh;
        let lower = self.bound_wrist || self.bound_neck;
        !(upper && lower)
    }

    pub fn mv(&self) -> i32 {
        // 步行距离
        // 从跑和跳两种模式中选一个最大的
        // 跑的模式：敏捷 * 步行系数
        // 跳的模式：敏捷 * 跳跃系数
        // 最终 / 5

        let walk = self.walk_coefficient(self.agi());
        let jump = self.jump_coefficient(self.agi());
        let mv = walk.max(jump);
        mv / 5
    }

    pub fn hand_str(&self) -> i32 {
        // 手部力量相关
        // 力量 * 手部自由度系数
        self.freedom_hand_coefficient(self.str())
    }

    pub fn hand_dex(&self) -> i32 {
        // 手部操作相关灵巧
        // 灵巧 * 手部自由度系数
        self.freedom_hand_coefficient(self.dex())
    }

    pub fn hand_agi(&self) -> i32 {
        // 手部操作相关敏捷
        // 敏捷 * 手部自由度系数
        self.freedom_hand_coefficient(self.agi())
    }

    pub fn unbound_force_upper(&self) -> i32 {
        // 上肢蛮力挣脱力
        // 独立占比
        // 五花  胳膊  悬挂
        // 50%   25%  25%
        // 如果joint，最终再除2，如果wrist，最终再除2
        
        let mut r = 0;
        if self.bound_neck {
            r += 2;
        }
        if self.bound_arm {
            r += 1;
        }
        if self.bound_hang {
            r += 1;
        }
        let mut re = self.str() * (4 - r) / 4;
        if self.bound_joint {re /= 2}
        if self.bound_wrist {re /= 2}
        re.min(100)
    }

    pub fn unbound_force_lower(&self) -> i32 {
        // 下肢蛮力挣脱力
        // 对于解绑脚腕
        // 基础概率50%
        // 独立占比
        // 大腿  小腿
        // 50%   50%
        // 如果反弓，额外/2
        // 对于解绑小腿
        // 基础概率50%，如果大腿被绑再/2
        if self.bound_ankle {
            let mut m2 = 2;
            if self.bound_thigh {m2 -= 1;}
            if self.bound_calve {m2 -= 1;}
            let mut r = self.str() * m2 / 2 / 2;
            if self.bound_long || self.bound_joint {r /= 2;}
            r
        }else if self.bound_calve{
            let mut r = self.str() / 2;
            if self.bound_thigh {r /= 2;}
            r
        }else if self.bound_thigh { 
            self.str() / 2 
        } else {
            self.str()
        }
    }

    
}