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

To run the client, you need to run:

`cargo run --bin client`

If you want to create a client using a language other than Rust, you can use the [protobuffer](./proto/dbscanserving.proto) for that.

There is also an implementation in [Python](https://github.com/carlosnatalino/dbscan-serving-python).

## TODO:

- [ ] Implement gRPC health service
- [ ] Implement the REST server
- [ ] Improve performance of the DBSCAN algorithm
- [ ] Implement other distance metrics (inspired by [this](https://scikit-learn.org/stable/modules/generated/sklearn.cluster.DBSCAN.html))
