use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

fn escape_time(c : Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    return None;
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(idx) => {
            match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }

        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20",','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("",','), None);
    assert_eq!(parse_pair::<i64>("10,20",','), Some((10,20)));
    assert_eq!(parse_pair::<f64>("1.0 2.0", ' '), Some((1.0, 2.0)));

}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex{re, im}),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex("1.25, -0.0625"), None);
    assert_eq!(parse_complex(",2.3"), None);
}

fn pixel_to_point(bounds: (usize, usize), 
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    let a = upper_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let b = upper_left.im - pixel.1 as f64 * height / bounds.1 as f64;

    return Complex{re: a, im: b};
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex {re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}),
    Complex{re: -0.5, im: -0.75}   );
}

fn render(pixels: &mut[u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) 
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let pt = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = 
                match escape_time(pt, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                }
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} filename pixels upperleft lowerright", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing pixel size");
    let upper_left = parse_complex(&args[3]).expect("error parsing upperleft");
    let lower_right = parse_complex(&args[4]).expect("error parsing lowerright");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing file");
}