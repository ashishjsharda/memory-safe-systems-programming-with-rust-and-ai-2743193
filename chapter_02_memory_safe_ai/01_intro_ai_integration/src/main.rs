use smartcore::neighbors::knn_classifier::{KNNClassifier, KNNClassifierParameters};
use smartcore::linalg::basic::matrix::DenseMatrix;


fn main() {
    // 1. Create a dataset
    // We'll use a simple 2D dataset where the first half of the points belong to class 0
    // and the second half belong to class 1.
    let x_data = DenseMatrix::from_2d_array(&[
        &[1.0, 1.0], &[1.2, 1.3], &[1.5, 1.1],
        &[8.0, 8.0], &[8.5, 8.2], &[8.1, 8.8]
    ]);
    let y_data = vec![0, 0, 0, 1, 1, 1];


    // 2. Train the KNN Classifier
    // Create a new KNN classifier with k=3 (it will consider the 3 nearest neighbors).
    let knn = KNNClassifier::fit(&x_data, &y_data, KNNClassifierParameters::default().with_k(3)).unwrap();


    // 3. Make a prediction
    // Let's predict the class of a new data point at [1.1, 1.2].
    // This point is close to the first three points in our dataset, which are all class 0.
    let x_test = DenseMatrix::from_2d_array(&[&[1.1, 1.2]]);
    let prediction = knn.predict(&x_test).unwrap();


    // 4. Print the result
    println!("The predicted class for [1.1, 1.2] is: {:?}", prediction);
}
