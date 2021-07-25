// #[disable(unused_imports)]
use linfa::traits::Transformer;
use linfa_clustering::{Dbscan};
// use linfa_clustering::generate_blob;
use linfa_clustering::generate_blobs;
// use ndarray::{Axis, array, s};
use ndarray::array;
use ndarray;
use ndarray_rand::rand::SeedableRng;
use rand_isaac::Isaac64Rng;
// use approx::assert_abs_diff_eq;
use ndarray::{s, Array1, Array2, ArrayBase, Data, Ix1, Ix2};
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;

// pub fn generate_blob(
//     blob_size: usize,
//     blob_centroid: &Array1<impl Data<Elem = f64>>,
//     rng: &mut impl Rng,
// ) -> Array2<f64> {
//     let shape = (blob_size, blob_centroid.len());
//     let origin_blob: Array2<f64> = Array::random_using(shape, StandardNormal, rng);
//     origin_blob + blob_centroid
// }

// pub fn generate_blobs(
//     blob_size: usize,
//     blob_centroids: &Array2<impl Data<Elem = f64>>,
//     rng: &mut impl Rng,
// ) -> Array2<f64> {
//     let (n_centroids, n_features) = blob_centroids.dim();
//     let mut blobs: Array2<f64> = Array2::zeros((n_centroids * blob_size, n_features));

//     for (blob_index, blob_centroid) in blob_centroids.genrows().into_iter().enumerate() {
//         let blob = generate_blob(blob_size, &blob_centroid, rng);

//         let indexes = s![blob_index * blob_size..(blob_index + 1) * blob_size, ..];
//         blobs.slice_mut(indexes).assign(&blob);
//     }
//     blobs
// }


fn main() {
    // Our random number generator, seeded for reproducibility
    let seed = 42;
    let mut rng = Isaac64Rng::seed_from_u64(seed);

    // `expected_centroids` has shape `(n_centroids, n_features)`
    // i.e. three points in the 2-dimensional plane
    let expected_centroids = array![[0., 1.], [-10., 20.], [-1., 10.]];
    // let expected_centroids: ndarray::Array2<f32> = arr.into_dimensionality::<Ix2>().unwrap();
    // let expected_centroids = arr2(&[[0., 1.], [-10., 20.], [-1., 10.]]);
    // Let's generate a synthetic dataset: three blobs of observations
    // (100 points each) centered around our `expected_centroids`
    let observations = generate_blobs(100, 
        &expected_centroids, 
        &mut rng);

    // Let's configure and run our DBSCAN algorithm
    // We use the builder pattern to specify the hyperparameters
    // `min_points` is the only mandatory parameter.
    // If you don't specify the others (e.g. `tolerance`)
    // default values will be used.
    let min_points = 3;
    let clusters = Dbscan::params(min_points)
        .tolerance(1e-2)
        .transform(&observations);
    // Points are `None` if noise `Some(id)` if belonging to a cluster.
    println!("Clusters: {:?}", clusters);

}