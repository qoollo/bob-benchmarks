#![allow(clippy::unwrap_used, clippy::significant_drop_tightening)]

use super::api::{bob_api_client::BobApiClient, Blob, BlobKey, BlobMeta};
use super::api::{DeleteRequest, GetRequest, PutRequest};
use criterion::measurement::WallTime;
use criterion::{BatchSize, BenchmarkGroup, BenchmarkId};
use tokio::runtime::Runtime;
use tonic::transport::Channel;
use ubyte::ToByteUnit;

pub fn read_blobs(c: &mut BenchmarkGroup<WallTime>, data: &[u8], addr: &str, id: &str) {
    let runtime = Runtime::new().unwrap();
    let client = runtime.block_on(initialize_client(addr));
    let datastruct = Some(Blob {
        data: data.into(),
        meta: Some(BlobMeta {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
    });
    let request = tonic::Request::new(super::api::PutRequest {
        key: Some(BlobKey {
            key: 1u64.to_ne_bytes().to_vec(),
        }),
        data: datastruct.clone(),
        options: None,
    });
    let client_handle = client.clone();
    runtime.block_on(async { put(client_handle, request).await });

    c.bench_function(BenchmarkId::new(id, data.len().bytes()), |b| {
        b.to_async(&runtime).iter_batched(
            || {
                let request = tonic::Request::new(super::api::GetRequest {
                    key: Some(BlobKey {
                        key: 1u64.to_ne_bytes().to_vec(),
                    }),
                    options: None,
                });

                (client.clone(), request, data)
            },
            |(client, req, data)| get(client, req, data),
            BatchSize::SmallInput,
        );
    });

    // Clean Up
    let request = tonic::Request::new(super::api::DeleteRequest {
        key: Some(BlobKey {
            key: 1u64.to_ne_bytes().to_vec(),
        }),
        meta: Some(BlobMeta {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
        options: None,
    });
    let client_handle = client.clone();
    runtime.block_on(async { delete(client_handle, request).await });
}

async fn get(mut client: BobApiClient<Channel>, req: tonic::Request<GetRequest>, _data: &[u8]) {
    // assert_eq!(data, client.get(req).await.unwrap().into_inner().data);
    client.get(req).await.unwrap();
}

async fn delete(mut client: BobApiClient<Channel>, req: tonic::Request<DeleteRequest>) {
    client.delete(req).await.unwrap();
}

pub fn write_blobs(c: &mut BenchmarkGroup<WallTime>, data: &[u8], addr: &str, id: &str) {
    let runtime = Runtime::new().unwrap();
    let client = runtime.block_on(initialize_client(addr));
    let datastruct = Some(Blob {
        data: data.into(),
        meta: Some(BlobMeta {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
    });

    c.bench_function(BenchmarkId::new(id, data.len().bytes()), |b| {
        b.to_async(&runtime).iter_batched(
            || {
                let request = tonic::Request::new(super::api::PutRequest {
                    key: Some(BlobKey {
                        key: 1u64.to_ne_bytes().to_vec(),
                    }),
                    data: datastruct.clone(),
                    options: None,
                });

                (client.clone(), request)
            },
            |(client, req)| put(client, req),
            BatchSize::SmallInput,
        );
    });

    // Clean Up
    let request = tonic::Request::new(super::api::DeleteRequest {
        key: Some(BlobKey {
            key: 1u64.to_ne_bytes().to_vec(),
        }),
        meta: Some(BlobMeta {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
        options: None,
    });
    let client_handle = client.clone();
    runtime.block_on(async { delete(client_handle, request).await });
}

async fn put(mut client: BobApiClient<Channel>, req: tonic::Request<PutRequest>) {
    client.put(req).await.unwrap();
}

async fn initialize_client(addr: &str) -> BobApiClient<Channel> {
    BobApiClient::connect(format!("http://{addr}:{}", "20000"))
        .await
        .unwrap().max_decoding_message_size(1024*1024*1024).max_encoding_message_size(1024*1024*1024)
}
