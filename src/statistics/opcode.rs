use crate::super_enum;

super_enum! {
    enum Opcode {
        Query => (0, "QUERY"),
        IQuery => (1, "IQUERY"),
        Status => (2, "STATUS"),
        Notify => (4, "NOTIFY"),
        Update => (5, "UPDATE"),
        Dso => (6, "DSO"),
    }
}
