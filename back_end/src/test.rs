use rand::Rng;
fn main(){
    let mut rng = rand::rng();
    for i in 1..2_000_000 {
        let x = rng.random_range(-1000..1000);
        if x < 0 {
            return;
        }
    }
}