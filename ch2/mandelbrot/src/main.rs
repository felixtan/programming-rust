use std::str::FromStr;
use std::fs::File;
use std::env;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;

// generic function; T is any type that implements FromStr trait
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            // parse s left and right of separator at index
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                // if both elements of the tuple are Ok variants of the Result type, return the tuple
                (Ok(l), Ok(r)) => Some((l, r)),

                // else, ignore
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

// parse a string coordinate and return a complex number if successful
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                            upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,   // tuple accessed using 0, 1
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.75 })
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                }
        }
    }
}

// returns Result value with () on Ok and std::io:Error on Err
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    /*
        ? is shorthand for common pattern:

        let output = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                return Err(e);
            }
        }
    */
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

    // on success, return unit type ()
    Ok(())
}

fn main() {
    // collect builds a vector from the iterator
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    // macro vec![v; n] creates n-length vector of value v 
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        // chunks_mut returns an iterator of mutable, non-overlapping slices of the buffer
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

        // ensures all threads have completed before it returns
        crossbeam::scope(|spawner| {
            // iterator gives each iteration of loop body exclusive ownership of a buffer band so one thread can write at a time
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                // creat a thread
                // move indicates the closure takes ownership of the variables it uses
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
        // unwrap so that if a thread panics, the program panics
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}