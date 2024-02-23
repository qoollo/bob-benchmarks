#![allow(clippy::unwrap_used, clippy::significant_drop_tightening)]

use criterion::measurement::WallTime;
use criterion::{BatchSize, BenchmarkGroup, BenchmarkId};
use minio::s3::args::{
    BucketExistsArgs, GetObjectArgs, MakeBucketArgs, PutObjectArgs, RemoveObjectArgs,
    UploadObjectArgs,
};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use tokio::runtime::Runtime;
use ubyte::ToByteUnit;

pub fn read_blobs(c: &mut BenchmarkGroup<WallTime>, mut data: &[u8], addr: &str, id: &str) {
    let runtime = Runtime::new().unwrap();
    let base_url = format!("http://{addr}").parse::<BaseUrl>().unwrap();
    let size = data.len();
    let client = Client::new(
        base_url,
        Some(Box::new(StaticProvider::new(
            "minioadmin",
            "minioadmin",
            None,
        ))),
        None,
        None,
    )
    .unwrap();

    let bucket_name = "test";

    // Check 'test' bucket exist or not.
    let exists = runtime
        .block_on(client.bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap()))
        .unwrap();

    // Make 'test' bucket if not exist.
    if !exists {
        runtime
            .block_on(client.make_bucket(&MakeBucketArgs::new(bucket_name).unwrap()))
            .unwrap();
    }

    let size = data.len();
    // runtime
    //     .block_on(
    //         client.upload_object(
    //             &UploadObjectArgs::new(
    //                 bucket_name,
    //                 "junk1M",
    //                 "/home/archeoss/playground/bob-test/junk1M",
    //             )
    //             .unwrap(),
    //         ),
    //     )
    //     .unwrap();
    runtime
        .block_on(client.put_object(
            &mut PutObjectArgs::new(bucket_name, "test-data", &mut data, Some(size), None).unwrap(),
        ))
        .unwrap();
    println!("tests");
    c.bench_function(BenchmarkId::new(id, size.bytes()), |b| {
        b.to_async(&runtime).iter_batched(
            || {
                let request = GetObjectArgs::new(bucket_name, "test-data").unwrap();
                // counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                (client.clone(), request)
            },
            |(client, req)| criterion::black_box(get(client, req)),
            BatchSize::SmallInput,
        );
    });

    // Clean Up
    // for num in 0..=u64::from(u8::MAX) {
    let request = RemoveObjectArgs::new(bucket_name, "test-data").unwrap();
    runtime.block_on(client.remove_object(&request)).unwrap();
    // }
}

async fn get(client: Client, req: GetObjectArgs<'_>) {
    let _data = client.get_object(&req).await.unwrap().bytes().await.unwrap();
   // dbg!(_data.len());
}

pub fn write_blobs(c: &mut BenchmarkGroup<WallTime>, data: &[u8], addr: &str, id: &str) {
    // pub fn write_blobs(
    //     c: &mut BenchmarkGroup<WallTime>,
    //     filename: &str,
    //     size: ByteUnit,
    //     addr: &str,
    //     id: &str,
    // ) {
    let runtime = Runtime::new().unwrap();
    let base_url = format!("http://{addr}").parse::<BaseUrl>().unwrap();
    let client = Client::new(
        base_url,
        Some(Box::new(StaticProvider::new(
            "minioadmin",
            "minioadmin",
            None,
        ))),
        None,
        None,
    )
    .unwrap();

    let bucket_name = "test";

    // Check 'test' bucket exist or not.
    let exists = runtime
        .block_on(client.bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap()))
        .unwrap();

    // Make 'test' bucket if not exist.
    if !exists {
        runtime
            .block_on(client.make_bucket(&MakeBucketArgs::new(bucket_name).unwrap()))
            .unwrap();
    }

    c.bench_function(BenchmarkId::new(id, data.len().bytes()), |b| {
        b.to_async(&runtime).iter_batched(
            || {
                // let size = data.len();
                (client.clone(), data, bucket_name)
            },
            // // |(client, data, bucket_name)| put(client, data, bucket_name),
            // //
            // (client.clone(), filename, bucket_name)
            // },
            |(client, data, bucket_name)| put(client, data, bucket_name),
            BatchSize::SmallInput,
        );
    });

    // Clean Up
    // for num in 0..=u64::from(u8::MAX) {
    let request = RemoveObjectArgs::new(bucket_name, "test-data").unwrap();
    // let request = RemoveObjectArgs::new(bucket_name, filename).unwrap();
    runtime.block_on(client.remove_object(&request)).unwrap();
    // }
}

async fn put(client: Client, mut data: &[u8], bucket_name: &str) {
    let size = data.len();
    client
        .put_object(
            &mut PutObjectArgs::new(bucket_name, "test-data", &mut data, Some(size), None).unwrap(),
        )
        .await
        .unwrap();
}

async fn upload(client: Client, filename: &str, bucket_name: &str) {
    // let size = data.len();
    client
        .upload_object(&UploadObjectArgs::new(bucket_name, filename, filename).unwrap())
        .await
        .unwrap();
}
