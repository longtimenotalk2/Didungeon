mod can;
mod eva;
mod exe;

fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

enum UnboundType {
    ForceUpper,
    ForceLower,
    Hand,
    Cuter,
}

