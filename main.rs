use std::f64::consts::PI;
use std::io;
use std::f64;
use colored;
use colored::Colorize;
use rand;
use rand::thread_rng;
use rand::Rng;

fn square(x:f64)->f64{
    x*x
}

fn gaussian(x:f64, stddev_o:f64, avg_u: f64) -> f64{
    let variance = square(stddev_o);
    let mut exponent = square(x-avg_u)/(2.0*variance);
    exponent=f64::exp(-exponent);
    exponent/(stddev_o*f64::sqrt(2.0*PI))
}

fn fill_2Dmatrix_gaussian_noisy(matrix:&mut Vec<Vec<f64>>, stddev_o:f64, avg_u:f64, num_particles:i64, noisepercent:f64){
    let centery:i32= matrix.len() as i32/2;
    let centerx:i32= matrix[0].len() as i32/2;
    let mut x:i32;
    let mut y:i32;
    let mut gaussx:f64;
    let mut gaussy:f64;
    let mut gauss_prob:f64;
    for i in (0..matrix.len()){
        for j in (0..matrix[0].len()){
            x=(j as i32) -centerx;
            y=(i as i32) -centery;
            gaussx=gaussian(x as f64, stddev_o, avg_u);
            gaussy=gaussian(y as f64, stddev_o, avg_u);
            gauss_prob=gaussx*gaussy;
            matrix[i][j]=gauss_prob*(num_particles as f64);
        }
    }
    let mut rng = thread_rng();
    for i in (0..matrix.len()){
        for j in (0..matrix[0].len()){
            if(rng.gen_range(0.0..=1.0)*100.0<noisepercent){
                matrix[i][j]+=matrix[i][j]*(rng.gen_range(0.0..=1.0)-0.5)*1.5;
            }
        }
    }
}

fn printgaussianmatr(matrix:&Vec<Vec<f64>>, stddev_o:f64, avg_u:f64, num_particles:i64){
    let mut maxavg:f64=square(gaussian(avg_u, stddev_o, avg_u))*num_particles as f64;
    let step=maxavg/6.0;
    let mut value:f64;
    for i in (0..matrix.len()){
        for j in (0..matrix[0].len()){
            value=matrix[i][j];
            if(value < 0.5){
                print!("{}", "  ".on_black());
            }
            else if(value>=0.5 && value< 1.0){
                print!("{{}}");
            }
            else if(value >=1.0 && value< step){
                print!("{}", "  ".on_cyan());
            }
            else if(value >=step && value< step* 2.0){
                print!("{}","  ".on_blue());
            }
            else if(value >=step*2.0 && value< step*3.0){
                print!("{}","  ".on_green());
            }
            else if(value >=step*3.0 && value< step*4.0){
                print!("{}","  ".on_yellow());
            }
            else if(value >=step*4.0 && value< step*5.0){
                print!("{}","  ".on_red());
            }
            else if(value >=step*5.0){
                print!("{}","  ".on_white());
            }
            else{
                print!("@@");
            }
        }
        println!();
    }
}

fn main() {
    let num_particles=11000;
    let stddev = 17.0;
    let noisepercent = 50.0;
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; 150]; 150];
    fill_2Dmatrix_gaussian_noisy(&mut matrix, stddev, 0.0, num_particles, noisepercent);
    printgaussianmatr(&matrix, stddev, 0.0, num_particles);
}
