use super::{Unit, Bound};

impl Unit {
    pub fn str(&self) -> i32 {
        (self.str_max - self.inj / self.inj_decay_rate).max(0)
    }

    pub fn dex(&self) -> i32 {
        (self.dex_max - self.inj / self.inj_decay_rate).max(0)
    }

    pub fn agi(&self) -> i32 {
        (self.agi_max - self.inj / self.inj_decay_rate).max(0)
    }

    // helper
    fn stand_walk_decay(&self, attr : i32) -> i32 {
        // 倒地为0
        // 渐进
        // 脚腕  小腿  大腿  自由
        // 25%   50%  75%   100%
        if self.fall {return 0;}
        let r = if self.bound_ankle {
            1
        }else if self.bound_calve {
            2
        }else if self.bound_thigh {
            3
        }else {4};
        attr * r / 4
    }

    fn stand_balance_decay(&self, mut attr : i32) -> i32 {
        // 倒地为0
        // 渐进
        // 脚腕  小腿  大腿  自由
        // 0%    25%   50%   100%
        if self.fall || self.bound_ankle {return 0};
        if self.bound_thigh {attr /= 2};
        if self.bound_calve {attr /= 2};
        attr
    }

    fn freedom_upper_decay(&self, mut attr : i32) -> i32 {
        // 上肢自由度：
        // 被驷马或者悬挂则为0
        // 手腕和五花独立除2
        if self.bound_joint || self.bound_hang {return 0};
        if self.bound_wrist {attr /= 2;}
        if self.bound_neck {attr /= 2;}
        attr
    }

    fn freedom_lower_decay(&self, attr : i32) -> i32 {
        // 自由度下肢
        // 渐进
        // 反弓  脚腕or小腿  大腿  自由
        // 0%   50%         75%   100%
        if self.bound_joint || self.bound_long {return 0};
        if self.bound_ankle || self.bound_calve {
            attr / 2
        } else if self.bound_thigh {
            attr * 3 / 4
        }else{
            attr
        }
    }

    // about bound

    pub fn unbound_force_upper(&self) -> i32 {
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

    pub fn unbound_hand_dex(&self) -> i32 {
        // 胳膊被绑就减半，两层减至1/4
        if self.bound_wrist {return 0;}
        let mut r = 1;
        if self.bound_neck {
            r *= 2;
        }
        if self.bound_arm {
            r *= 2;
        }
        self.dex() / r
    }

    pub fn unbound_hand_agi(&self) -> i32 {
        // 胳膊被绑就减半，两层减至1/4
        if self.bound_wrist {return 0;}
        let mut r = 1;
        if self.bound_neck {
            r *= 2;
        }
        if self.bound_arm {
            r *= 2;
        }
        self.agi() / r
    }

    pub fn tie_power(&self) -> i32 {
        // 灵巧
        if self.bound_wrist {return 0;}
        let mut r = self.dex();
        if self.bound_neck {r /= 2};
        r
    }

    pub fn tie_spd(&self) -> i32 {
        if self.bound_wrist {return 0;}
        let mut r = self.agi();
        if self.bound_neck {r /= 2};
        r
    }

    pub fn anti_tie_upper(&self) -> i32 {
        self.freedom_upper_decay(self.str())
    }

    pub fn anti_tie_lower(&self) -> i32 {
        self.freedom_lower_decay(self.str())
    }

    // about movable

    pub fn acc_hand(&self) -> i32 {
        // 手腕被绑直接清零
        // 给一个站立移动的衰减
        // 如果胳膊被绑则减半
        if self.bound_wrist {return 0;}
        let mut r = self.stand_walk_decay(self.dex());
        if self.bound_neck {r /= 2};
        r
    }

    pub fn evd_body(&self) -> i32 {
        // 敏捷的站立平衡系数和灵巧的1/4比最大值
        // 上肢被绑需要*3/4
        // 倒地为0
        if self.fall {return 0;}
        let mut r = self.stand_balance_decay(self.agi()).max(self.dex() / 4);
        if self.bound_neck || self.bound_wrist {
            r = r * 3 / 4;
        }
        r
    }

    pub fn thrust(&self) -> i32 {
        // 推力，用于计算推倒时使用
        // 基础为力量做站立移动衰减
        // 如果手腕被绑清零，胳膊被绑减半
        if self.bound_wrist {return 0;}
        let mut r = self.stand_balance_decay(self.str());
        if self.bound_neck {r /= 2};
        r
    }

    pub fn anti_thrust(&self) -> i32 {
        // 反推力，用于计算推倒时使用
        // 力量的站立平衡系数和灵巧的1/4比最大值
        // 上肢被绑需要*3/4
        // 倒地为0
        if self.fall {return 0;}
        let mut r = self.stand_balance_decay(self.str()).max(self.dex() / 4);
        if self.bound_neck || self.bound_wrist {
            r = r * 3 / 4;
        }
        r
    }

    pub fn downforce(&self) -> i32 {
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

    pub fn anti_downforce(&self) -> i32 {
        // 上肢下肢求和
        // 上肢需要完全自由，否则为0
        // 下肢则看自由度
        let upper = if self.bound_wrist || self.bound_neck {0} else {1};
        (self.freedom_lower_decay(self.str()) + upper * self.str()) / 2
    }

    pub fn can_stand(&self) -> bool {
        // 必须上肢下肢有一边完全自由才行，可以100%起身
        // 否则只能杂技起身，暂时不考虑
        let upper = self.bound_ankle || self.bound_calve || self.bound_thigh;
        let lower = self.bound_wrist || self.bound_neck;
        !(upper && lower)
    }

    pub fn spd(&self) -> i32 {
        // 上下肢自由度求和
        (self.freedom_lower_decay(self.agi()) + 
        self.freedom_upper_decay(self.agi()) ) / 2
    }

    // where to unbound

    pub fn next_force_upper(&self) -> Option<Bound> {
        if self.bound_wrist {
            Some(Bound::Wrist)
        }else{
            None
        }
    }

    pub fn next_force_lower(&self) -> Option<Bound> {
        if self.bound_ankle {
            Some(Bound::Ankle)
        }else if self.bound_calve {
            Some(Bound::Calve)
        }else{
            None
        }
    }

    pub fn next_hand(&self) -> Option<Bound> {
        if self.bound_wrist {
            None
        }else{
            if self.bound_arm {
                Some(Bound::Arm)
            }else if self.bound_neck {
                Some(Bound::Neck)
            }else if self.bound_ankle {
                Some(Bound::Ankle)
            }else if self.bound_calve {
                Some(Bound::Calve)
            }else if self.bound_thigh {
                Some(Bound::Thigh)
            }else{
                None
            }
        }
    }

    pub fn next_can_tie_choices(&self) -> Vec<(Bound, bool)> {
        let mut list = vec!();

        if !self.bound_wrist {list.push((Bound::Wrist, true));}
        if !self.bound_ankle {list.push((Bound::Ankle, true));}
        if !self.bound_hang && !self.bound_joint && self.bound_neck && self.bound_wrist {
            list.push((Bound::Hang, true));
        }
        if !self.bound_hang && !self.bound_joint && self.bound_wrist && self.bound_ankle {
            list.push((Bound::Joint, true));
        }
        if !self.bound_neck {list.push((Bound::Neck, true));}
        if !self.bound_arm && self.bound_neck {list.push((Bound::Arm, true));}
        if !self.bound_long && !self.bound_joint && self.bound_ankle && self.bound_wrist {list.push((Bound::Long, true));}
        if !self.bound_calve {list.push((Bound::Calve, true));}
        if !self.bound_thigh {list.push((Bound::Thigh, true));}
        if !self.bound_long && self.bound_joint && self.bound_ankle && self.bound_wrist {list.push((Bound::Long, true));}
        

        let should_release_joint = self.bound_neck && self.bound_arm && !self.bound_hang && self.bound_wrist && self.bound_joint && self.bound_thigh && self.bound_calve && self.bound_ankle && self.bound_long;
        if should_release_joint {list.push((Bound::Joint, false));}
        list
    }

    pub fn is_defeated(&self) -> bool {
        self.bound_neck && self.bound_arm && self.bound_hang && self.bound_wrist && !self.bound_joint && self.bound_thigh && self.bound_calve && self.bound_ankle && self.bound_long
    }


}