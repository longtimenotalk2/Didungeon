mod can;
mod eva;
mod exe;

fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

fn to_dmg(dmg_o : i32, min_dmg_set : i32) -> i32 {
    dmg_o.max(min_dmg_set)
}

enum UnboundType {
    ForceUpper,
    ForceLower,
    Hand,
    Cuter,
}

