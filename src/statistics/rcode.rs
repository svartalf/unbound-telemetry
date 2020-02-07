use crate::super_enum;

super_enum! {
    enum Rcode {
        NoError => (0, "NOERROR"),
        FormErr => (1, "FORMERR"),
        ServFail => (2, "SERVFAIL"),
        NXDomain => (3, "NXDOMAIN"),
        NotImpl => (4, "NOTIMPL"),
        Refused => (5, "REFUSED"),
        YXDomain => (6, "YXDOMAIN"),
        YXRRSet => (7, "YXRRSET"),
        NXRRSet => (8, "NXRRSET"),
        NotAuth => (9, "NOTAUTH"),
        NotZone => (10, "NOTZONE"),
    }
}
