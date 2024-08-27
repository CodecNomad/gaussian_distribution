use std::f64::consts::PI;
use std::f64;
use colored;
use colored::Colorize;
use rand;
use rand::thread_rng;
use rand::Rng;

fn square(x: f64) -> f64 {
    return f64::powi(x, 2)
}

fn gaussian(x:f64, stddev_o:f64, avg_u: f64) -> f64{
    let variance = square(stddev_o);
    let mut exponent = square(x-avg_u)/(2.0*variance);
    exponent=f64::exp(-exponent);
    exponent/(stddev_o*f64::sqrt(2.0*PI))
}

fn fill_2d_matrix_gaussian_noisy(matrix:&mut Vec<Vec<f64>>, stddev_o:f64, avg_u:f64, num_particles:i64, noisepercent:f64){
    let centery:i32= matrix.len() as i32/2;
    let centerx:i32= matrix[0].len() as i32/2;
    let mut x:i32;
    let mut y:i32;
    let mut gaussx:f64;
    let mut gaussy:f64;
    let mut gauss_prob:f64;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            x=(j as i32) -centerx;
            y=(i as i32) -centery;
            gaussx=gaussian(x as f64, stddev_o, avg_u);
            gaussy=gaussian(y as f64, stddev_o, avg_u);
            gauss_prob=gaussx*gaussy;
            matrix[i][j]=gauss_prob*(num_particles as f64);
        }
    }
    let mut rng = thread_rng();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if rng.gen_range(0.0..=1.0)*100.0<noisepercent {
                matrix[i][j]+=matrix[i][j]*(rng.gen_range(0.0..=1.0)-0.5)*1.5;
            }
        }
    }
}

fn print_gaussian_matrix(matrix:&Vec<Vec<f64>>, stddev_o:f64, avg_u:f64, num_particles:i64){
    let maxavg:f64=square(gaussian(avg_u, stddev_o, avg_u))*num_particles as f64;
    let step=maxavg/6.0;
    let mut value:f64;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            value=matrix[i][j];
            match value {
                v if v < 0.5 => {
                    print!("{}", "  ".on_black());
                },
                v if v >= 0.5 && v < 1.0 => {
                    print!("{{}}");
                },
                v if v >= 1.0 && v < step => {
                    print!("{}", "  ".on_cyan());
                },
                v if v >= step && v < step * 2.0 => {
                    print!("{}", "  ".on_blue());
                },
                v if v >= step * 2.0 && v < step * 3.0 => {
                    print!("{}", "  ".on_green());
                },
                v if v >= step * 3.0 && v < step * 4.0 => {
                    print!("{}", "  ".on_yellow());
                },
                v if v >= step * 4.0 && v < step * 5.0 => {
                    print!("{}", "  ".on_red());
                },
                v if v >= step * 5.0 => {
                    print!("{}", "  ".on_white());
                },
                _ => {
                    print!("@@");
                }
            }
        }
        println!();
    }
}

fn main() {
    let num_particles=11000;
    let std_dev = 17.0;
    let noisepercent = 50.0;
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; 150]; 150];
    fill_2d_matrix_gaussian_noisy(&mut matrix, std_dev, 0.0, num_particles, noisepercent);
    print_gaussian_matrix(&matrix, std_dev, 0.0, num_particles);
}
