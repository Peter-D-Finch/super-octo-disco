use std::process::Command;
//use std::{thread, time::Duration};
use std::str;
use crate::stats_plot::plot_timeseries;

mod stats {
	/*
	TODO:
	 - Spearman Rank
	 - rolling_stats
	 - Approximate Entropy
	*/
	#![allow(dead_code)]
	pub fn mean(data: &Vec<f64>) -> f64 {
		let mut sum = 0.0;
		for i in 0..data.len() {
			sum = sum + data[i];
		}
		let avg = sum / (data.len() as f64);
		return avg;
	}
	pub fn covariance(a: Vec<f64>, b: Vec<f64>) -> f64 {
		assert!(a.len() == b.len());
		let a_avg = mean(&a);
		let b_avg = mean(&b);
		let mut sum = 0.0;
		for i in 0..a.len() {
			sum = sum + ((a[i] - a_avg)*(b[i] - b_avg));
		}
		let covariance = sum / (a.len() as f64);
		return covariance;
	}
	pub fn st_dev(data: Vec<f64>) -> f64 {
		let avg = mean(&data);
		let mut diff: Vec<f64> = Vec::new();
		for i in 0..data.len() {
			diff.push(f64::powf(data[i] - avg, 2.0));
		}
		let avg_diff = mean(&diff);
		let st_dev: f64 = avg_diff.sqrt();
		return st_dev;
	}
	pub fn pearson(a: &Vec<f64>, b: &Vec<f64>) -> f64{
		assert!(a.len() == b.len());
		let cov = covariance(a.clone(), b.clone());
		let a_stdev = st_dev(a.clone());
		let b_stdev = st_dev(b.clone());
		let correl = cov / (a_stdev * b_stdev);
		return correl;
	}
	pub fn z_score(x: f64, mean: f64, stdev: f64) -> f64 {
		return (x - mean) / stdev;
	}
	pub fn z_score_vec(data: Vec<f64>) -> Vec<f64> {
		let avg = mean(&data);
		let stdev = st_dev(data.clone());
		let mut result = Vec::new();
		for i in 0..data.len() {
			result.push(z_score(data[i], avg, stdev));
		}
		return result;
	}
	pub fn remove_outliers(data: Vec<f64>, n: f64, iter: i64) -> Vec<f64> {
		let mut result: Vec<f64> = Vec::new();
		let z_scores = z_score_vec(data.clone());
		let stdev = st_dev(data.clone());
		let avg = mean(&data);
		for i in 0..z_scores.len() {
			if z_scores[i] > n {
				result.push(avg + (stdev * n));
			}
			else if z_scores[i] < (n * -1.0) {
				result.push(avg - (stdev * n));
			}
			else {
				result.push(data[i]);
			}
		}
		for _j in 0..iter {
			let z_scores_adj = z_score_vec(result.clone());
			let stdev_adj = st_dev(result.clone());
			let avg_adj = mean(&result);
			for i in 0..z_scores_adj.len() {
				if z_scores_adj[i] > n {
					result[i] = avg_adj + (stdev_adj * n);
				}
				else if z_scores_adj[i] < (n * -1.0) {
					result[i] = avg_adj - (stdev_adj * n);
				}
			}
		}
		return result;
	}
	pub fn minmax_scale(data: &Vec<f64>) -> Vec<f64> {
		let min = data.iter().fold(0.0/0.0, |m, v| v.min(m));
		let max = data.iter().fold(0.0/0.0, |m, v| v.max(m));
		let mut result: Vec<f64> = Vec::new();
		for i in 0..data.len() {
			result.push((data[i] - min) / (max - min));
		}
		return result;
	}
	fn make_points(a: &Vec<f64>, b: &Vec<f64>) -> Vec<(f64, f64)> {
		assert!(a.len() == b.len());
		let mut result: Vec<(f64, f64)> = Vec::new();
		for i in 0..a.len() {
			result.push((a[i], b[i]));
		}
		return result;
	}
}

mod stats_plot {
	#![allow(dead_code)]
	use textplots::{Chart, Shape, Plot};
	use crate::stats::minmax_scale;
	pub fn plot_scatter(x: &Vec<f64>, y: &Vec<f64>) {
		let a = minmax_scale(&x);
		let b = minmax_scale(&y);
		let points = &make_points(a, b)[..];
		println!("-----------------------------------------------------------------------------------------------------");
		Chart::new(180,120,-0.1,1.1).lineplot(&Shape::Points(points)).display();
		println!("-----------------------------------------------------------------------------------------------------");
	}
	pub fn plot_timeseries(q: &Vec<f64>) {
		println!("-----------------------------------------------------------------------------------------------------");
		Chart::new(180,60,1.0,q.len() as f32).lineplot(&Shape::Continuous(Box::new(|x| q[(x as usize)%q.len()] as f32))).display();
		println!("-----------------------------------------------------------------------------------------------------");
	}
	fn make_points(a: Vec<f64>, b: Vec<f64>) -> Vec<(f32, f32)> {
		assert!(a.len() == b.len());
		let mut result: Vec<(f32, f32)> = Vec::new();
		for i in 0..a.len() {
			result.push((a[i] as f32, b[i] as f32));
		}
		return result;
	}
}

fn get_val() -> i64{
	let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
				.args(["/C", "run.bat"])
				.output()
				.expect("failed to execute process")
	} else {
		Command::new("sh")
				.arg("-c")
				.arg("echo hello")
				.output()
				.expect("failed to execute process")
	};

	let result = output.stdout;
	return str::from_utf8(&result[118..result.len()-3]).unwrap().parse::<i64>().expect("error converting to int");
}

fn get_vec(n: i64) -> Vec<i64> {
	let mut result: Vec<i64> = Vec::new();
	for _i in 0..n {
		result.push(get_val());
		//thread::sleep(Duration::from_millis(100));
	}
	return result;
}

fn main() {
	let data = get_vec(300);
	let mut data2: Vec<f64> = Vec::new();
	for i in 0..data.len() {
		data2.push(data[i] as f64);
	}
	plot_timeseries(&data2);
}


