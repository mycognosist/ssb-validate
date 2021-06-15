use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flumedb::OffsetLog;
use ssb_validate::message::{par_validate_message_hash_chain_of_feed, validate_message_hash_chain};

pub fn verify_bench(c: &mut Criterion) {
    let in_log = OffsetLog::<u32>::open_read_only("./test_vecs/piet.offset").unwrap();

    let msgs = in_log
        .iter()
        .map(|entry| entry.data)
        .take(2)
        .collect::<Vec<_>>();

    c.bench_function("verify", |b| {
        b.iter(|| {
            let res =
                validate_message_hash_chain::<_, &[u8]>(black_box(msgs[1].clone()), Some(&msgs[0]));
            assert!(res.is_ok());
        })
    });
}

pub fn par_verify_messages_bench(c: &mut Criterion) {
    let in_log = OffsetLog::<u32>::open_read_only("./test_vecs/piet.offset").unwrap();

    let msgs = in_log
        .iter()
        .map(|entry| entry.data)
        .take(1000)
        .collect::<Vec<_>>();

    c.bench_function("par_verify_batch", |b| {
        b.iter(|| {
            let res = par_validate_message_hash_chain_of_feed::<_, &[u8]>(black_box(&msgs), None);
            assert!(res.is_ok());
        })
    });
}

criterion_group! {
    name = verify_batch;
    config = Criterion::default().sample_size(10);
    targets = par_verify_messages_bench
}
criterion_group!(verify_single, verify_bench);

criterion_main!(verify_batch, verify_single);
