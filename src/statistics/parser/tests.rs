use claim::{assert_ok, assert_some_eq};

use super::Parser;
use crate::statistics::{Class, Rcode, Rtype};

static STATS: &str = include_str!("../../../assets/test_text_stats.txt");
static STATS_1_13_2: &str = include_str!("../../../assets/test_text_stats_1_13_2.txt");

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

    assert_some_eq!(stats.query_types.get(&Rtype::from_int(0)), &21838);
    assert_some_eq!(stats.query_types.get(&Rtype::A), &4576639648);
    assert_some_eq!(stats.query_types.get(&Rtype::Ns), &195714);
    assert_some_eq!(stats.query_types.get(&Rtype::Mf), &1);
    assert_some_eq!(stats.query_types.get(&Rtype::Cname), &223477);
    assert_some_eq!(stats.query_types.get(&Rtype::Soa), &252841);
    assert_some_eq!(stats.query_types.get(&Rtype::Mr), &2);
    assert_some_eq!(stats.query_types.get(&Rtype::Null), &238628);
    assert_some_eq!(stats.query_types.get(&Rtype::Wks), &230227);
    assert_some_eq!(stats.query_types.get(&Rtype::Ptr), &8446520);
    assert_some_eq!(stats.query_types.get(&Rtype::Hinfo), &231428);
    assert_some_eq!(stats.query_types.get(&Rtype::Mx), &34204);
    assert_some_eq!(stats.query_types.get(&Rtype::Txt), &510353);
    assert_some_eq!(stats.query_types.get(&Rtype::Aaaa), &151992709);
    assert_some_eq!(stats.query_types.get(&Rtype::Nxt), &3);
    assert_some_eq!(stats.query_types.get(&Rtype::Srv), &1883446);
    assert_some_eq!(stats.query_types.get(&Rtype::Naptr), &1578821);
    assert_some_eq!(stats.query_types.get(&Rtype::Ds), &215);
    assert_some_eq!(stats.query_types.get(&Rtype::Dnskey), &252);
    assert_some_eq!(stats.query_types.get(&Rtype::from_int(65)), &140375490);
    assert_some_eq!(stats.query_types.get(&Rtype::from_int(96)), &1);
    assert_some_eq!(stats.query_types.get(&Rtype::Any), &82558);

    assert_some_eq!(stats.query_classes.get(&Class::from_int(0)), &2);
    assert_some_eq!(stats.query_classes.get(&Class::In), &4882932525);
    assert_some_eq!(stats.query_classes.get(&Class::Ch), &1073);
    assert_some_eq!(stats.query_classes.get(&Class::Hs), &3);
    assert_some_eq!(stats.query_classes.get(&Class::from_int(5)), &1);

    assert_eq!(stats.num_query_tcp, 0);
    assert_eq!(stats.num_query_tcp_out, 2);
    assert_eq!(stats.num_query_tls, 0);
    assert_eq!(stats.num_query_tls_resume, 0);
    assert_eq!(stats.num_query_ipv6, 0);
    assert_eq!(stats.num_query_https, 10);

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

    assert_some_eq!(stats.answer_rcodes.get(&Rcode::NoError), &1315);
    assert_some_eq!(stats.answer_rcodes.get(&Rcode::FormErr), &0);
    assert_some_eq!(stats.answer_rcodes.get(&Rcode::ServFail), &0);
    assert_some_eq!(stats.answer_rcodes.get(&Rcode::NXDomain), &23);
    assert_some_eq!(stats.answer_rcodes.get(&Rcode::NotImp), &0);
    assert_some_eq!(stats.answer_rcodes.get(&Rcode::Refused), &0);

    assert_eq!(stats.http.query_buffer, 1024);
    assert_eq!(stats.http.response_buffer, 2048);

    assert_eq!(stats.threads.len(), 2);
}

#[test]
fn test_parser_1_13_2_format() {
    let parser = Parser::new();

    assert_ok!(parser.parse(STATS_1_13_2));
}
