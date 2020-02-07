use crate::super_enum;

super_enum! {
    enum Class {
        In => (1, "IN"),
        Ch => (3, "CH"),
        Hs => (4, "HS"),
        None => (0xFE, "NONE"),
        Any => (0xFF, "ANY"),
    }
}
