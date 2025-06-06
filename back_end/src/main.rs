use rand::Rng;
fn main() {
    let start = std::time::Instant::now();
    let mut rng = rand::rng();
    let mut test: [[i64; 3]; 3] = [[100, 0, 50], [-50, 50, 50], [-50, -50, 50]];
    let mut d: [i64; 3] = [0, 0, 100];
    println!("{}", intersect(&test, &d));
    let mut count: i32 = 0;
    for i in 1..2_000_000 {
        let randTri = [[rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)], [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)], [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)]];
        let randD = [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)];
        if intersect(&randTri, &randD) == -1.0 {
            count = count + 1;
        }
    }
    println!("{}", count);
    println!("{}", start.elapsed().as_millis());
}
fn determinant(mat: &[[i64; 3]; 3]) -> i64 {
    mat[0][0] * (mat[1][1] * mat [2][2] - mat[2][1] * mat[1][2]) - mat[0][1] * (mat[1][0] * mat [2][2] - mat[2][0] * mat[1][2]) + mat[0][2] * (mat[1][0] * mat [2][1] - mat[2][0] * mat[1][1])
}
fn diff(v1: &[i64; 3], v2: &[i64; 3]) -> [i64; 3]{
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
fn transpose(mat: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    [[mat[0][0], mat[1][0], mat[2][0]], [mat[0][1], mat[1][1], mat[2][1]], [mat[0][2], mat[1][2], mat[2][2]]]
}
fn print_arr(mat: &[[i64; 3]; 3]){
    println!("{} {} {}", mat[0][0], mat[0][1], mat[0][2]);
    println!("{} {} {}", mat[1][0], mat[1][1], mat[1][2]);
    println!("{} {} {}", mat[2][0], mat[2][1], mat[2][2]);
}
fn intersect(triangle: &[[i64; 3]; 3], ray: &[i64; 3]) -> f32 {
    let ab = diff(&triangle[0], &triangle[1]);
    let ac = diff(&triangle[0], &triangle[2]);
    let mut arr: [[i64; 3]; 3] = [*ray, ab, ac];
    arr = transpose(&arr);
    let det_base: f32 = determinant(&arr) as f32;
    if det_base.abs() < 0.0001 {
        return -1.0;
    }
    arr = transpose(&arr);
    arr[0] = triangle[0];
    arr = transpose(&arr);
    let c: f32 = determinant(&arr) as f32 / det_base;
    arr = transpose(&arr);
    arr[0] = *ray;
    arr[1] = triangle[0];
    arr = transpose(&arr);
    let u: f32 = determinant(&arr) as f32 / det_base;
    arr = transpose(&arr);
    arr[1] = ab;
    arr[2] = triangle[0];
    arr = transpose(&arr);
    let v: f32 = determinant(&arr) as f32 / det_base;
    if u > 0.0 {
        if v > 0.0 {
            if u + v < 1.0 {
                return c;
            }
        }
    }
    return -1.0;

}