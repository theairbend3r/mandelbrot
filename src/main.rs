use num::Complex;

fn generate_mandelbrot_values(canvas_width: usize, canvas_height: usize, max_iter: usize) {
    let xmin = -2.0;
    let xmax = 1.0;
    let ymin = -1.0;
    let ymax = 1.0;

    for y_fig in 0..canvas_height {
        let mut row = String::with_capacity(canvas_width);
        for x_fig in 0..canvas_width {
            let x_scaled = xmin + (xmax - xmin) * (x_fig as f64 / canvas_width as f64);
            let y_scaled = ymin + (ymax - ymin) * y_fig as f64 / canvas_height as f64;
            let mandelbrot_iter = calc_mandelbrot_iter(x_scaled, y_scaled, max_iter);
            row.push(convert_to_symbol(mandelbrot_iter));
        }
        println!("{}", row);
    }
}

fn calc_mandelbrot_iter(x: f64, y: f64, max_iter: usize) -> usize {
    let c = Complex::new(x, y);
    let mut z = Complex { re: 0.0, im: 0.0 };

    for itr_idx in 0..max_iter {
        if z.norm() > 2.0 {
            return itr_idx;
        }
        z = z * z + c;
    }

    max_iter
}

fn convert_to_symbol(mandelbrot_iter: usize) -> char {
    match mandelbrot_iter {
        0..=1 => ' ',
        2..=4 => '.',
        5..=8 => '`',
        9..=16 => ':',
        17..=32 => '-',
        33..=64 => '~',
        65..=128 => '=',
        129..=256 => '+',
        257..=512 => '*',
        513..=800 => 'o',
        801..=1024 => '%',
        1025..=2048 => '#',
        _ => '@',
    }
}

fn main() {
    let terminal_size = crossterm::terminal::size();
    let mut canvas_width = 30;
    let mut canvas_height = 10;

    match terminal_size {
        Ok((width, height)) => {
            canvas_width = width - 5;
            canvas_height = height - 5;
        }
        Err(_) => {
            println!("creating with default height and width.");
        }
    }

    let max_iter = 2048;
    generate_mandelbrot_values(canvas_width as usize, canvas_height as usize, max_iter);
}
