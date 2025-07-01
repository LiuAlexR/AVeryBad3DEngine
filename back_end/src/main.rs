use rand::Rng;

use minifb::{Key, Window, WindowOptions};
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
    let camera: [[i64; 3]; 5] = [
        [0, 0, 0],
        
        [-200, -200, 100],
        [-200, 200, 100],
        [200, 200, 100],

        [200, -200, 100],
    ]; //Counterclockwise, starting from bottom right
    let d0 = &camera[1];
    let dx = diff(&camera[4], &camera[1]);
    println!("dx: {} {} {}", dx[0], dx[1], dx[2]);
    let dy = diff(&camera[2], &camera[1]);
    println!("dy: {} {} {}", dy[0], dy[1], dy[2]);
    let MAX_HEIGHT: i64 = 480;
    let HEIGHT = 480;
    let WIDTH = 640;
    let MAX_WIDTH: i64 = 640;
    let mut ray: [i64; 3] = [camera[1][0], camera[1][1], camera[1][2]];

    // println!("|‾‾‾‾‾‾‾‾‾|");
    // for height in 1..MAX_HEIGHT {
    //     let rayV = add(d0, &mult(&dy, (height as f64) / (MAX_HEIGHT as f64)));
    //     let mut count = 1;
    //     print!("|");
    //     for width in 1..MAX_WIDTH{
    //         let rayVH = add(&rayV, &mult(&dx, (width as f64) / (MAX_WIDTH as f64)));
    //         if intersect(&test, &rayVH) != -1.0 /*|| intersect(&test2, &rayVH) != -1.0*/{
    //             count = count + 1;
    //             print!("*");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     print!("|");
    //     println!()
    // }
    // println!("|_________|");
    // println!("{}", start.elapsed().as_micros() as f64 / 1000.0);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Populate the buffer with a color pattern.
    // This example creates a simple gradient pattern.
    // The outer loop iterates through each row (y-coordinate).
    for y in 0..HEIGHT {
        let rayV = add(d0, &mult(&dy, (y as f64) / (MAX_HEIGHT as f64)));
        // The inner loop iterates through each column (x-coordinate).
        for x in 0..WIDTH {
            let rayVH = add(&rayV, &mult(&dx, (x as f64) / (MAX_WIDTH as f64)));
            // Calculate the index in the 1D buffer for the current (x, y) coordinate.
            let index = y * WIDTH + x;
            if intersect(&test, &rayVH) != -1.0
            /*|| intersect(&test2, &rayVH) != -1.0*/
            {
                buffer[index] = 0xFF000000;
            } else {
                buffer[index] = 0xFFFFFFFF;
            }
        }
    }

    // Create a new window with the specified dimensions and title.
    // WindowOptions::default() uses default settings (e.g., resizable, no border).
    let mut window = Window::new(
        "Rust GUI Color Display - ESC to exit", // Window title
        WIDTH,                                  // Window width
        HEIGHT,                                 // Window height
        WindowOptions::default(),               // Default window options
    )
    .unwrap_or_else(|e| {
        // Handle potential errors during window creation.
        panic!("{}", e);
    });

    // Main event loop: keep the window open and update its content until the user closes it.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window with the current pixel buffer.
        // This draws the generated color pattern onto the window.
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                // Handle potential errors during buffer update.
                eprintln!("Error updating buffer: {}", e);
            });
    }
}
fn determinant(mat: &[[i64; 3]; 3]) -> i64 {
    mat[0][0] * (mat[1][1] * mat[2][2] - mat[2][1] * mat[1][2])
        - mat[0][1] * (mat[1][0] * mat[2][2] - mat[2][0] * mat[1][2])
        + mat[0][2] * (mat[1][0] * mat[2][1] - mat[2][0] * mat[1][1])
}
fn diff(v1: &[i64; 3], v2: &[i64; 3]) -> [i64; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}
fn add(v1: &[i64; 3], v2: &[i64; 3]) -> [i64; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}
fn mult(v1: &[i64; 3], num: f64) -> [i64; 3] {
    [
        (v1[0] as f64 * num) as i64,
        (v1[1] as f64 * num) as i64,
        (v1[2] as f64 * num) as i64,
    ]
}
fn transpose(mat: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    [
        [mat[0][0], mat[1][0], mat[2][0]],
        [mat[0][1], mat[1][1], mat[2][1]],
        [mat[0][2], mat[1][2], mat[2][2]],
    ]
}
fn print_arr(mat: &[[i64; 3]; 3]) {
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
