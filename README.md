# Rust implementation of DBSCAN Serving

DBSCAN Serving aims to be a gRPC and REST implementation of the DBSCAN algorithm inspired by TensorFlow Serving.

The idea came from the following posts
- https://www.lpalmieri.com/posts/2019-12-01-taking-ml-to-production-with-rust-a-25x-speedup/
- https://rust-ml.github.io/book/4_dbscan.html
- https://github.com/LukeMathWalker/clustering-benchmarks

The DBSCAN implementation was based upon the code available here:
- https://blog.petrzemek.net/2017/01/01/implementing-dbscan-from-distance-matrix-in-rust/
- https://github.com/s3rvac/blog/tree/master/en-2017-01-01-implementing-dbscan-from-distance-matrix-in-rust

## TODO:
- Implement gRPC health service
- Implement the REST server
- Improve performance of the DBSCAN algorithm
- Implement other distance metrics (inspired by https://scikit-learn.org/stable/modules/generated/sklearn.cluster.DBSCAN.html)
