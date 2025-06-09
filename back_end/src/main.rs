use rand::Rng;
fn main() {
    let start = std::time::Instant::now();
    let mut rng = rand::rng();
    let test: [[i64; 3]; 3] = [[-50, -50, 50], [-50, 50, 50], [50, -50, 50]];
    let test2: [[i64; 3]; 3] = [[50, 50, 50], [-50, 50, 50], [50, -50, 50]];
    let d: [i64; 3] = [0, 0, 100];
    {
    /*println!("{}", determinant(&[d, diff(&test[0], &test[1]), diff(&test[0], &test[2])]));
    println!("{}", intersect(&test, &d));
    let mut count: i32 = 0;
    for i in 1..2_000_000 {
        let randTri = [[rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)], [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)], [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)]];
        let randD = [rng.random_range(-1000..1000), rng.random_range(-1000..1000), rng.random_range(-1000..1000)];
        if intersect(&randTri, &randD) == -1.0 {
            count = count + 1;
        }
    }
    println!("{}", count);*/
    }
    let camera: [[i64; 3]; 5] = [[0, 0, 0], [200, -200, 100], [-200,-200, 100], [-200, 200, 100], [200, 200, 100]]; //Counterclockwise, starting from bottom right
    let d0 = &camera[1];
    let dx = diff(&camera[4], &camera[1]);
    println!("dx: {} {} {}", dx[0], dx[1], dx[2]);
    let dy = diff(&camera[2], &camera[1]);
    println!("dy: {} {} {}", dy[0], dy[1], dy[2]);
    let MAX_HEIGHT: i64 = 480;
    let MAX_WIDTH: i64 = 640;
    let mut ray: [i64; 3] = [camera[1][0], camera[1][1], camera[1][2]];
    println!("|‾‾‾‾‾‾‾‾‾|");
    for height in 1..MAX_HEIGHT {
        let rayV = add(d0, &mult(&dy, (height as f64) / (MAX_HEIGHT as f64)));
        let mut count = 1;
        print!("|");
        for width in 1..MAX_WIDTH{
            let rayVH = add(&rayV, &mult(&dx, (width as f64) / (MAX_WIDTH as f64)));
            if intersect(&test, &rayVH) != -1.0 /*|| intersect(&test2, &rayVH) != -1.0*/{
                count = count + 1;
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!()
    }
    println!("|_________|");
    println!("{}", start.elapsed().as_micros() as f64 / 1000.0);
}
fn determinant(mat: &[[i64; 3]; 3]) -> i64 {
    mat[0][0] * (mat[1][1] * mat [2][2] - mat[2][1] * mat[1][2]) - mat[0][1] * (mat[1][0] * mat [2][2] - mat[2][0] * mat[1][2]) + mat[0][2] * (mat[1][0] * mat [2][1] - mat[2][0] * mat[1][1])
}
fn diff(v1: &[i64; 3], v2: &[i64; 3]) -> [i64; 3]{
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
fn add(v1: &[i64; 3], v2: &[i64; 3]) -> [i64; 3]{
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}
fn mult(v1: &[i64; 3], num: f64) -> [i64; 3]{
    [(v1[0] as f64 * num) as i64, (v1[1] as f64 * num) as i64, (v1[2]  as f64 * num) as i64]
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
    //arr = transpose(&arr);
    let det_base: f32 = determinant(&arr) as f32;
    if det_base.abs() < 0.0001 {
        return -1.0;
    }
    //arr = transpose(&arr);
    arr[0] = triangle[0];
    //arr = transpose(&arr);
    let c: f32 = determinant(&arr) as f32 / det_base;
    if c < 0.0001 || c > 1.0001 {
        return -1.0;
    }
    //arr = transpose(&arr);
    arr[0] = *ray;
    arr[1] = triangle[0];
    //arr = transpose(&arr);
    let u: f32 = determinant(&arr) as f32 / det_base;
    if u < 0.0 {
        return -1.0;
    }
    //arr = transpose(&arr);
    arr[1] = ab;
    arr[2] = triangle[0];
    //arr = transpose(&arr);
    let v: f32 = determinant(&arr) as f32 / det_base;
    if v < 0.0 {
        return -1.0;
    }
    if u + v < 1.0001 {
        return c;
    }
    return -1.0;

}
