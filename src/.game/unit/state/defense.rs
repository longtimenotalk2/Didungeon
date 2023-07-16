use crate::game::unit::Unit;

impl Unit {
    pub fn evd(&self) -> i32 {
        // 闪避率
        // 从跑和跳两种模式中选一个最大的
        // 跑的模式：敏捷 * 步行系数
        // 跳的模式：敏捷 * 跳跃系数

        let walk = self.walk_coefficient(self.agi());
        let jump = self.jump_coefficient(self.agi());
        walk.max(jump)
    }

    pub fn anti_push(&self) -> i32 {
        // 反推力，用于计算推倒时使用
        // 从跑和跳两种模式中选一个最大的
        // 跑的模式：力量 * 平衡系数
        // 跳的模式：力量 * 跳跃系数
        // 跌倒为0
        
        if self.fall {return 0;}
        let walk = self.balance_coefficient(self.str());
        let jump = self.jump_coefficient(self.str());
        walk.max(jump)
    }

    pub fn anti_hold(&self) -> i32 {
        // 反下压力
        // 上肢下肢自由度取平均
        let upper = self.freedom_upper_coefficient(self.str());
        let lower = self.freedom_lower_coefficient(self.str());
        (upper + lower) / 2
    }

    pub fn anti_tie_upper(&self) -> i32 {
        // 上肢反抗被捆绑力
        self.freedom_upper_coefficient(self.str())
    }

    pub fn anti_tie_lower(&self) -> i32 {
        // 下肢反抗被捆绑力
        self.freedom_lower_coefficient(self.str())
    }

    

    

    
}