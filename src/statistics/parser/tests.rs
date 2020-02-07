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

//num.query.ratelimited=0
//num.answer.secure=70
//num.answer.bogus=0
//num.rrset.bogus=1
//num.query.aggressive.NOERROR=0
//num.query.aggressive.NXDOMAIN=0
//unwanted.queries=0
//unwanted.replies=0
//msg.cache.count=885
//rrset.cache.count=1019
//infra.cache.count=2
//key.cache.count=196
//dnscrypt_shared_secret.cache.count=0
//dnscrypt_nonce.cache.count=0
//num.query.dnscrypt.shared_secret.cachemiss=0
//num.query.dnscrypt.replay=0
//num.query.authzone.up=0
//num.query.authzone.down=0
//num.query.subnet=0
//num.query.subnet_cache=0

//histogram.000000.000000.to.000000.000001=22
//histogram.000000.000001.to.000000.000002=0
//histogram.000000.000002.to.000000.000004=0
//histogram.000000.000004.to.000000.000008=0
//histogram.000000.000008.to.000000.000016=0
//histogram.000000.000016.to.000000.000032=0
//histogram.000000.000032.to.000000.000064=0
//histogram.000000.000064.to.000000.000128=0
//histogram.000000.000128.to.000000.000256=0
//histogram.000000.000256.to.000000.000512=0
//histogram.000000.000512.to.000000.001024=0
//histogram.000000.001024.to.000000.002048=0
//histogram.000000.002048.to.000000.004096=32
//histogram.000000.004096.to.000000.008192=98
//histogram.000000.008192.to.000000.016384=55
//histogram.000000.016384.to.000000.032768=152
//histogram.000000.032768.to.000000.065536=286
//histogram.000000.065536.to.000000.131072=207
//histogram.000000.131072.to.000000.262144=205
//histogram.000000.262144.to.000000.524288=55
//histogram.000000.524288.to.000001.000000=3
//histogram.000001.000000.to.000002.000000=1
//histogram.000002.000000.to.000004.000000=2
//histogram.000004.000000.to.000008.000000=0
//histogram.000008.000000.to.000016.000000=0
//histogram.000016.000000.to.000032.000000=0
//histogram.000032.000000.to.000064.000000=0
//histogram.000064.000000.to.000128.000000=0
//histogram.000128.000000.to.000256.000000=0
//histogram.000256.000000.to.000512.000000=0
//histogram.000512.000000.to.001024.000000=0
//histogram.001024.000000.to.002048.000000=0
//histogram.002048.000000.to.004096.000000=0
//histogram.004096.000000.to.008192.000000=0
//histogram.008192.000000.to.016384.000000=0
//histogram.016384.000000.to.032768.000000=0
//histogram.032768.000000.to.065536.000000=0
//histogram.065536.000000.to.131072.000000=0
//histogram.131072.000000.to.262144.000000=0
//histogram.262144.000000.to.524288.000000=0
