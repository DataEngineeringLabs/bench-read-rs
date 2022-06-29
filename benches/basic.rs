use criterion::{criterion_group, criterion_main, Criterion};

use bench_read::*;

fn add_benchmark(c: &mut Criterion) {
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);

        let lengths = (0..10).map(|x| x + size).collect::<Vec<_>>();

        let mut data = vec![];
        for length in lengths.clone() {
            data.extend_from_slice(&((length) as u32).to_le_bytes());
            data.extend(std::iter::repeat((length % 255) as u8).take(length));
        }

        c.bench_function(&format!("read0 2^{}", log2_size), |b| {
            b.iter(|| {
                let reader = std::io::Cursor::new(&data);
                let a = read_many0(reader).collect::<Result<Vec<_>, _>>().unwrap();
                assert!(a.iter().zip(lengths.iter()).all(|(x, y)| x.0 == *y));
            })
        });

        c.bench_function(&format!("read1 2^{}", log2_size), |b| {
            b.iter(|| {
                let reader = std::io::Cursor::new(&data);
                let a = read_many1(reader).collect::<Result<Vec<_>, _>>().unwrap();
                assert!(a.iter().zip(lengths.iter()).all(|(x, y)| x.0 == *y));
            })
        });

        c.bench_function(&format!("read2 2^{}", log2_size), |b| {
            b.iter(|| {
                let reader = std::io::Cursor::new(&data);
                let a = read_many2(reader).collect::<Result<Vec<_>, _>>().unwrap();
                assert!(a.iter().zip(lengths.iter()).all(|(x, y)| x.0 == *y));
            })
        });

        c.bench_function(&format!("read3 2^{}", log2_size), |b| {
            b.iter(|| {
                let reader = std::io::Cursor::new(&data);
                let a = read_many3(reader).collect::<Result<Vec<_>, _>>().unwrap();
                assert!(a.iter().zip(lengths.iter()).all(|(x, y)| x.0 == *y));
            })
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
