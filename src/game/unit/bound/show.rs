use colorful::{Color, Colorful, core::color_string::CString};

use std::fmt::Write;
use super::BoundPart;
use super::BoundPart::Neck as Neck;
use super::BoundPart::Arm as Arm;
use super::BoundPart::Hang as Hang;
use super::BoundPart::Wrist as Wrist;
use super::BoundPart::Joint as Joint;
use super::BoundPart::Thigh as Thigh;
use super::BoundPart::Calve as Calve;
use super::BoundPart::Ankle as Ankle;
use super::BoundPart::Long as Long;

use super::BoundState;

impl BoundPart {
    pub fn name(&self) -> &'static str {
        match self {
            BoundPart::Neck => "五花",
            BoundPart::Arm => "大臂",
            BoundPart::Hang => "手腕<-->后颈",
            BoundPart::Wrist => "手腕",
            BoundPart::Joint => "脚腕<-->手腕",
            BoundPart::Thigh => "大腿",
            BoundPart::Calve => "小腿",
            BoundPart::Ankle => "脚腕",
            BoundPart::Long => "脚腕<-->后颈",
        }
    }
}

impl BoundState {
    pub fn identity(&self, bound_case : Option<(&BoundPart, bool)>, show_loose : bool) -> String {
        let mut new = self.clone();
        let color = match bound_case {
            Some((_, true)) => Color::Green,
            Some((_, false)) => Color::Red,
            None => Color::White,
        };
        if let Some((bound, true)) = bound_case {
            new.tie(bound);
        }

        let creator = |cases : Vec<(&str, bool, bool, bool)>| -> String {
            let mut s: CString = " ".to_string().color(Color::White);
            for (look, crit, color_crit, loose_crit) in cases {
                if crit {
                    let r = look.to_string();
                    if color_crit {
                        s = r.color(color);
                    }else if loose_crit && show_loose {
                        s = r.color(Color::Yellow);
                    }else{
                        s = r.color(Color::White);
                    }
                    break;
                }
            }
            s.to_string()
        };

        let itb = |bd : &BoundPart| -> bool {
            match bound_case {
                Some((b, _)) => {
                    if b == bd {
                        true
                    } else {
                        false
                    }
                },
                None => false,
            }
        };

        let mut s = "[".to_string();
        // neck
        write!(s, "{}", creator(
            vec![
                ("@", new.is_bound_neck(), itb(&Neck), new.is_loose_neck()),
            ]
        )).unwrap();
        // arm
        write!(s, "{}", creator(
            vec![
                ("O", new.is_bound_arm(), itb(&Arm), new.is_loose_arm()),
                ("=", new.is_bound_hang() && new.is_bound_long(), itb(&Hang), new.is_loose_hang()),
                ("-", new.is_bound_hang(), itb(&Hang), new.is_loose_hang()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // hang
        write!(s, "{}", creator(
            vec![
                ("=", new.is_bound_hang() && new.is_bound_long(), itb(&Hang), new.is_loose_hang()),
                ("-", new.is_bound_hang(), itb(&Hang), new.is_loose_hang()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // wrist
        write!(s, "{}", creator(
            vec![
                ("@", new.is_bound_wrist(), itb(&Wrist), new.is_loose_wrist()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // joint
        write!(s, "{}", creator(
            vec![
                ("=", new.is_bound_joint() && new.is_bound_long(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_joint(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // thigh
        write!(s, "{}", creator(
            vec![
                ("0", new.is_bound_thigh(), itb(&Thigh), new.is_loose_thigh()),
                ("=", new.is_bound_joint() && new.is_bound_long(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_joint(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // calve
        write!(s, "{}", creator(
            vec![
                ("O", new.is_bound_calve(), itb(&Calve), new.is_loose_calve()),
                ("=", new.is_bound_joint() && new.is_bound_long(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_joint(), itb(&Joint), new.is_loose_joint()),
                ("-", new.is_bound_long(), itb(&Long), new.is_loose_long()),
            ]
        )).unwrap();
        // ankle
        write!(s, "{}", creator(
            vec![
                ("@", new.is_bound_ankle(), itb(&Ankle), new.is_loose_ankle()),
            ]
        )).unwrap();
        s += "]";
        
        s
    }

    pub fn identity_tightness(&self, bound : &BoundPart) -> String {
        let tightness = self.get_tightness(bound);
        if 0 < tightness && tightness < 100 {
            let a = format!("({}%)", tightness);
            let mut s = String::new();
            write!(&mut s, "{}", a.color(Color::Yellow)).unwrap();
            s
        }else{
            String::new()
        }
    }
}