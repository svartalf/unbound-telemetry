use super::Parser;
use crate::statistics::{Class, Opcode, Rcode, Rtype};

static STATS: &str = include_str!("../../../assets/test_text_stats.txt");

#[test]
#[allow(clippy::cognitive_complexity)]
fn test_parser() {
    let parser = Parser::new();
    let result = parser.parse(STATS);
    let stats = result.unwrap();

    // TODO: approx eq for a whole value
    assert_eq!(stats.time.now.as_secs(), 1_580_162_982);
    assert_eq!(stats.time.up.as_secs(), 32_586);
    assert_eq!(stats.time.elapsed.as_secs(), 32_586);

    assert_eq!(stats.cache.rrset, 443_440);
    assert_eq!(stats.cache.message, 312_898);
    assert_eq!(stats.modules.iterator, 16_588);
    assert_eq!(stats.modules.validator, 140_392);
    assert_eq!(stats.modules.respip, 0);
    assert_eq!(stats.modules.subnet, 0);
    assert_eq!(stats.cache.dnscrypt_shared_secret, 0);
    assert_eq!(stats.cache.dnscrypt_nonce, 0);
    assert_eq!(stats.mem_streamwait, 0);
    assert_eq!(stats.query_types.get(&Rtype::A), Some(&981));
    assert_eq!(stats.query_types.get(&Rtype::Ptr), Some(&4));
    assert_eq!(stats.query_types.get(&Rtype::Aaaa), Some(&353));
    assert_eq!(stats.query_classes.get(&Class::In), Some(&1338));
    assert_eq!(stats.query_opcodes.get(&Opcode::Query), Some(&1338));
    assert_eq!(stats.num_query_tcp, 0);
    assert_eq!(stats.num_query_tcp_out, 2);
    assert_eq!(stats.num_query_tls, 0);
    assert_eq!(stats.num_query_tls_resume, 0);
    assert_eq!(stats.num_query_ipv6, 0);

    assert_eq!(stats.flags.qr, 0);
    assert_eq!(stats.flags.aa, 0);
    assert_eq!(stats.flags.tc, 0);
    assert_eq!(stats.flags.rd, 1338);
    assert_eq!(stats.flags.ra, 0);
    assert_eq!(stats.flags.z, 0);
    assert_eq!(stats.flags.ad, 0);
    assert_eq!(stats.flags.cd, 0);

    assert_eq!(stats.num_query_edns_present, 0);
    assert_eq!(stats.num_query_edns_do, 0);

    assert_eq!(stats.answer_rcodes.get(&Rcode::NoError), Some(&1315));
    assert_eq!(stats.answer_rcodes.get(&Rcode::FormErr), Some(&0));
    assert_eq!(stats.answer_rcodes.get(&Rcode::ServFail), Some(&0));
    assert_eq!(stats.answer_rcodes.get(&Rcode::NXDomain), Some(&23));
    assert_eq!(stats.answer_rcodes.get(&Rcode::NotImpl), Some(&0));
    assert_eq!(stats.answer_rcodes.get(&Rcode::Refused), Some(&0));

    assert_eq!(stats.threads.len(), 2);
}
