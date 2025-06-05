fn main() {
    let test = [[100, 0, 50], [-50, 50, 50], [-50, -50, 50]];
    let ab = diff(&test[0], &test[1]);
    let ac = diff(&test[0], &test[1]);
    let a = test[0];
    let d: [i32; 3] = [0, 0, 100];
    println!("{}", diff(&test[0], &test[1])[2]);
}
fn discriminant(mat: [[i32; 3]; 3]) -> i32 {
    mat[0][0] * (mat[1][1] * mat [2][2] - mat[2][1] * mat[1][2]) - mat[0][1] * (mat[1][0] * mat [2][2] - mat[2][0] * mat[1][2]) + mat[0][2] * (mat[1][0] * mat [2][1] - mat[2][0] * mat[1][1])
}
fn diff(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3]{
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
//fn intersect(triangle: [[i32; 3]; 3], ray: [i32; 3]) -> bool {
//
//}