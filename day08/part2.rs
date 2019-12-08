static INPUT: &'static str = include_str!(r"input.txt");

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let mut pixels = [' '; WIDTH * HEIGHT];
    for layer in INPUT.as_bytes().rchunks(WIDTH * HEIGHT) {
        for (pixel, digit) in pixels.iter_mut().zip(layer) {
            match digit {
                b'0' => *pixel = ' ',
                b'1' => *pixel = '*',
                _ => {}
            }
        }
    }   

    let final_out: String = pixels.chunks(WIDTH).flat_map(|row| 
                                  row.iter()
                                  .copied()
                                  .chain(std::iter::once('\n'))).collect();

    println!("{}", final_out);
}
