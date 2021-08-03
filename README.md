# Rust implementation of DBSCAN Serving

DBSCAN Serving aims to be a gRPC and REST implementation of the DBSCAN algorithm inspired by TensorFlow Serving.

The idea came from the following posts

- [Taking ML to Production with Rust](https://www.lpalmieri.com/posts/2019-12-01-taking-ml-to-production-with-rust-a-25x-speedup/)
- [DBSCAN](https://rust-ml.github.io/book/4_dbscan.html)
- [Clustering Benchmark](https://github.com/LukeMathWalker/clustering-benchmarks)

The DBSCAN implementation was based upon the code available here:

- [Implementing DBSCAN from distance matrix in Rust](https://blog.petrzemek.net/2017/01/01/implementing-dbscan-from-distance-matrix-in-rust/)
- [Implementing dbscan from a distance matrix in Rust (source code)](https://github.com/s3rvac/blog/tree/master/en-2017-01-01-implementing-dbscan-from-distance-matrix-in-rust)

# Using the client/server

To run the server, you need to run:

`cargo run --bin server`

or to run in release mode:

`cargo run --release --bin server`

To run the client, you need to run:

`cargo run --bin client`

or to run in release mode:

`cargo run --release --bin client`

If you want to create a client using a language other than Rust, you can use the [protobuffer](./proto/dbscanserving.proto) for that.

There is also an implementation in [Python](https://github.com/carlosnatalino/dbscan-serving-python).

# Performance

**The results reported below are obtained using a non-scientific method and if performance measures are critical to you you should run your own tests in your own platform.**
The results were obtained using an Intel 10875H processor running Windows and Ubuntu over WSL.
The requests have 310 samples per request, with 100 features per sample.
Both server and client were compiled in the `release` mode.

Using the Rust client available  in release mode the performance is as follows:
- gRPC: 5.05 ms per request
- REST: 6.1 ms per request

For comparison purposes, the [Python](https://github.com/carlosnatalino/dbscan-serving-python) server implementation performance obtained running the same Rust client was as follows:
- gRPC: 13.5 ms per request (Rust got 2.7x speed up)
- REST: 87.7 ms per request (Rust got ~14x speed up)

## TODO:

- [ ] Implement gRPC health service
- [x] Implement the REST server
- [ ] Improve performance of the DBSCAN algorithm
- [ ] Implement other distance metrics (inspired by [this](https://scikit-learn.org/stable/modules/generated/sklearn.cluster.DBSCAN.html))
