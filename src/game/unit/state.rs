use super::{Unit, Bound};
mod offense;
mod defense;
mod to_self;

impl Unit {
    pub fn str(&self) -> i32 {
        if self.stun {return 0;}
        (self.str_max - self.inj / self.inj_coefficient).max(0)
    }

    pub fn dex(&self) -> i32 {
        if self.stun {return 0;}
        (self.dex_max - self.inj / self.inj_coefficient).max(0)
    }

    pub fn agi(&self) -> i32 {
        if self.stun {return 0;}
        (self.agi_max - self.inj / self.inj_coefficient).max(0)
    }

    // helper
    fn walk_coefficient(&self, attr : i32) -> i32 {
        // 步行系数
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

    fn balance_coefficient(&self, mut attr : i32) -> i32 {
        // 平衡度系数
        // 倒地为0
        // 渐进
        // 脚腕  小腿  大腿  自由
        // 0%    25%   50%   100%
        // 上肢有绳索则减半
        if self.fall || self.bound_ankle {return 0};
        if self.bound_thigh {attr /= 2};
        if self.bound_calve {attr /= 2};
        if self.bound_neck || self.bound_wrist {attr /= 2};
        attr
    }

    fn jump_coefficient(&self, mut attr : i32) -> i32 {
        // 跳跃系数
        // 和双腿并拢跳跃有关的系数，倒地时与跳起站立有关
        // 如果反弓，则为0
        // 否则，基础值为50%，如上肢受限则为25%，倒地时额外减半
        if self.bound_joint || self.bound_long {
            return 0;
        }
        attr /= 2;
        if self.bound_wrist || self.bound_neck {
            attr /= 2;
        }
        if self.fall {
            attr /= 2;
        }
        attr
    }

    fn freedom_upper_coefficient(&self, mut attr : i32) -> i32 {
        // 上肢自由度：
        // 被驷马或者悬挂则为0
        // 手腕和五花独立除2
        if self.bound_joint || self.bound_hang {return 0};
        if self.bound_wrist {attr /= 2;}
        if self.bound_neck {attr /= 2;}
        attr
    }

    fn freedom_lower_coefficient(&self, attr : i32) -> i32 {
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

    fn freedom_hand_coefficient(&self, mut attr : i32) -> i32 {
        // 手的自由度
        // 手腕被绑则为0
        // 胳膊被绑就减半，两层减至1/4
        if self.bound_wrist {return 0;}
        if self.bound_neck {
            attr /= 2;
        }
        if self.bound_arm {
            attr /= 2;
        }
        attr
    }

    // where to unbound

    pub fn next_force_upper(&self) -> Option<Bound> {
        if self.bound_wrist {
            Some(Bound::Wrist)
        }else if self.bound_arm {
            Some(Bound::Arm)
        }else {
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

    pub fn next_untie(&self) -> Option<Bound> {
        if self.bound_wrist {
            Some(Bound::Wrist)
        }else if self.bound_ankle {
            Some(Bound::Ankle)
        }else if self.bound_arm {
            Some(Bound::Arm)
        }else if self.bound_neck {
            Some(Bound::Neck)
        }else if self.bound_calve {
            Some(Bound::Calve)
        }else if self.bound_thigh {
            Some(Bound::Thigh)
        }else{
            None
        }
    }

    pub fn next_can_tie_choices(&self) -> Vec<(Bound, bool)> {
        let mut list = vec!();

        let should_tie_hang = self.bound_neck && self.bound_arm && !self.bound_hang && self.bound_wrist && !self.bound_joint && self.bound_thigh && self.bound_calve && self.bound_ankle && self.bound_long;
        if should_tie_hang {
            return vec!((Bound::Hang, true));
        }

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

    pub fn block(&self) -> bool {
        self.anti_hold() > 0
    }

    pub fn is_defeated(&self) -> bool {
        self.bound_neck && self.bound_arm && self.bound_hang && self.bound_wrist && !self.bound_joint && self.bound_thigh && self.bound_calve && self.bound_ankle && self.bound_long
    }


}