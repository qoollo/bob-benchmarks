use criterion::{Criterion, Throughput};
static KB: usize = 1024;
static MB: usize = 1024 * KB;
static SIZES: [usize; 4] = [KB, 50 * KB, MB, 100 * MB];
// static SIZES: [usize; 1] = [100 * MB];
// static FILES: [&str; 4] = ["junk1K", "junk50K", "junk1M", "junk100M"];
static BOB_CLUSTERS: [(&str, &str); 2] = [
    ("bob: one-node", "192.168.17.17"),
    ("bob: two-node", "192.168.17.15"),
];
// static MINIO_CLUSTERS: [(&str, &str); 2] = [("one-node", "minio-11"), ("two-node", "minio-21")];
static MINIO_CLUSTERS: [(&str, &str); 2] = [
    ("minio: one-node", "localhost:9003"),
    ("minio: two-node", "localhost:9002"),
];
// static MINIO_CLUSTERS: [(&str, &str); 2] = [
//     ("minio: one-node", "172.24.0.7"),
//     ("minio: two-node", "172.24.0.5"),
// ];
pub mod api {
    tonic::include_proto!("bob_storage");
}

mod bob;
mod minio;

pub fn benches(c: &mut Criterion) {
    write_blobs(c);
    read_blobs(c);
}

pub fn write_blobs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Writes");

    for blob_size in SIZES {
        group.throughput(Throughput::Bytes(blob_size as u64));

        let blob = vec![42; blob_size as usize];

        #[cfg(feature = "one_node")]
        {
            let (id, addr) = BOB_CLUSTERS[0];
            bob::write_blobs(&mut group, &blob, addr, id);
            let (id, addr) = MINIO_CLUSTERS[0];
            minio::write_blobs(&mut group, &blob, addr, id);
        }
        #[cfg(feature = "two_node")]
        {
            let (id, addr) = BOB_CLUSTERS[1];
            bob::write_blobs(&mut group, &blob, addr, id);
            let (id, addr) = MINIO_CLUSTERS[1];
            minio::write_blobs(&mut group, &blob, addr, id);
        }
    }
}

pub fn read_blobs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reads");

    for blob_size in SIZES {
        group.throughput(Throughput::Bytes(blob_size as u64));

        let blob = vec![42; blob_size as usize];

        #[cfg(feature = "one_node")]
        {
            let (id, addr) = BOB_CLUSTERS[0];
            bob::read_blobs(&mut group, &blob, addr, id);
            let (id, addr) = MINIO_CLUSTERS[0];
            minio::read_blobs(&mut group, &blob, addr, id);
        }
        #[cfg(feature = "two_node")]
        {
            let (id, addr) = BOB_CLUSTERS[1];
            bob::read_blobs(&mut group, &blob, addr, id);
            let (id, addr) = MINIO_CLUSTERS[1];
            minio::read_blobs(&mut group, &blob, addr, id);
        }
    }
}
