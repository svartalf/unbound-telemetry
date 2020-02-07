#![allow(unused)]

// All functions in this module should be replaced with some already existing enums,
// as long as they will provide `from_primitive` and `as_static_str` methods.

/// Return query record type based on its number.
///
/// Based on the `rdata_field_descriptors[]` array from the `unbound/rrdef.c`,
/// used to match `ServerStats.qtype` field values to qtype names.
///
/// Implemented only partially right now with the most popular record types.
///
/// TODO: Complete the match.
pub fn rr_type(value: usize) -> Option<&'static str> {
    let type_ = match value {
        1 => "A",
        2 => "NS",
        5 => "CNAME",
        6 => "SOA",
        10 => "NULL",
        12 => "PTR",
        15 => "MX",
        16 => "TXT",
        24 => "SIG",
        25 => "KEY",
        28 => "AAAA",
        33 => "SRV",
        35 => "NAPTR",
        41 => "OPT",
        43 => "DS",
        44 => "SSHFP",
        46 => "RRSIG",
        47 => "NSEC",
        48 => "DNSKEY",
        50 => "NSEC3",
        51 => "NSEC3PARAM",
        52 => "TLSA",
        61 => "OPENPGPKEY",
        255 => "ANY",
        252 => "AXFR",
        257 => "CAA",
        _ => return None,
    };

    Some(type_)
}

pub fn rr_class(value: usize) -> Option<&'static str> {
    let class = match value {
        1 => "IN",
        3 => "CH",
        4 => "HS",
        254 => "NONE",
        255 => "ANY",
        _ => return None,
    };

    Some(class)
}

pub fn rr_opcode(value: usize) -> Option<&'static str> {
    let class = match value {
        0 => "QUERY",
        1 => "IQUERY",
        2 => "STATUS",
        4 => "NOTIFY",
        5 => "UPDATE",
        _ => return None,
    };

    Some(class)
}

pub fn rr_rcode(value: usize) -> Option<&'static str> {
    let rcode = match value {
        0 => "NOERROR",
        1 => "FORMERR",
        2 => "SERVFAIL",
        3 => "NXDOMAIN",
        4 => "NOTIMPL",
        5 => "REFUSED",
        6 => "YXDOMAIN",
        7 => "YXRRSET",
        8 => "NXRRSET",
        9 => "NOTAUTH",
        10 => "NOTZONE",
        _ => return None,
    };

    Some(rcode)
}
