use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, io::prelude::*, time::Instant};

fn main() {
    let file_path = "temp-file";
    let num_runs = 1000;
    let chunk_size = 1024 * 1024 * 10; // Set a smaller chunk size

    // Create the file before entering the loop
    File::create(file_path).unwrap();

    let pb = ProgressBar::new(num_runs);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg:.cyan} {pos:.2}/{len:.2}")
            .unwrap(),
    );

    let mut read_times: Vec<_> = Vec::new();
    let mut write_times: Vec<_> = Vec::new();

    for _ in 0..num_runs {
        let mut file = File::open(file_path).unwrap();

        let start = Instant::now();
        let mut buf = vec![0; chunk_size];
        while file.read(&mut buf).unwrap() > 0 {}
        let end = Instant::now();

        read_times.push(end - start);

        let mut file = File::create(file_path).unwrap();

        let start = Instant::now();
        for chunk in buf.chunks(chunk_size) {
            file.write(chunk).unwrap();
        }
        let end = Instant::now();

        write_times.push(end - start);

        pb.inc(1);
    }

    pb.finish();

    let avg_read_time = read_times
        .iter()
        .map(|t| t.as_millis() as u128)
        .sum::<u128>()
        / (num_runs as u128);
    let avg_write_time = write_times
        .iter()
        .map(|t| t.as_millis() as u128)
        .sum::<u128>()
        / (num_runs as u128);

    println!("Read {} bytes in {} ms on average", 1048576, avg_read_time);
    println!(
        "Wrote {} bytes in {} ms on average",
        1048576, avg_write_time
    );

    // Cleanup after finishing benchmark
    std::fs::remove_file(file_path).unwrap();
}
