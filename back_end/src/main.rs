fn main() {
    let test = [[-10, 0, 50], [-50, 50, 50], [-50, -50, 50]];
    let ab = diff(&test[0], &test[1]);
    let ac = diff(&test[0], &test[2]);
    let a = test[0];
    let d: [i32; 3] = [0, 0, 100];
    let mut arr: [[i32; 3]; 3] = [a, ab, ac];
    arr = transpose(&arr);
    println!("{}", intersect(&test, &d));
}
fn determinant(mat: &[[i32; 3]; 3]) -> i32 {
    mat[0][0] * (mat[1][1] * mat [2][2] - mat[2][1] * mat[1][2]) - mat[0][1] * (mat[1][0] * mat [2][2] - mat[2][0] * mat[1][2]) + mat[0][2] * (mat[1][0] * mat [2][1] - mat[2][0] * mat[1][1])
}
fn diff(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3]{
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
fn transpose(mat: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    [[mat[0][0], mat[1][0], mat[2][0]], [mat[0][1], mat[1][1], mat[2][1]], [mat[0][2], mat[1][2], mat[2][2]]]
}
fn printArr(mat: &[[i32; 3]; 3]){
    println!("{} {} {}", mat[0][0], mat[0][1], mat[0][2]);
    println!("{} {} {}", mat[1][0], mat[1][1], mat[1][2]);
    println!("{} {} {}", mat[2][0], mat[2][1], mat[2][2]);
}
fn intersect(triangle: &[[i32; 3]; 3], ray: &[i32; 3]) -> f32 {
    let ab = diff(&triangle[0], &triangle[1]);
    let ac = diff(&triangle[0], &triangle[2]);
    let mut arr: [[i32; 3]; 3] = [*ray, ab, ac];
    arr = transpose(&arr);
    let detBase: f32 = determinant(&arr) as f32;
    arr = transpose(&arr);
    arr[0] = triangle[0];
    arr = transpose(&arr);
    let c: f32 = determinant(&arr) as f32 / detBase;
    arr = transpose(&arr);
    arr[0] = *ray;
    arr[1] = triangle[0];
    arr = transpose(&arr);
    let u: f32 = determinant(&arr) as f32 / detBase;
    arr = transpose(&arr);
    arr[1] = ab;
    arr[2] = triangle[0];
    arr = transpose(&arr);
    let v: f32 = determinant(&arr) as f32 / detBase;
    if u > 0.0 {
        if v > 0.0 {
            if u + v < 1.0 {
                return c
            }
        }
    }
    -1.0

}