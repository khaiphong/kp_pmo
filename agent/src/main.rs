/*
 failed to load manifest for workspace member `/home/batien/Kp/GitSrc/github.com/khaiphong/kp_pmo/agent`
referenced by workspace at `/home/batien/Kp/GitSrc/github.com/khaiphong/kp_pmo/Cargo.toml`

Caused by:
  failed to load manifest for dependency `matrix`

Caused by:
  failed to read `/home/batien/Kp/GitSrc/github.com/khaiphong/kp_pmo/agent/agent/src/matrix/Cargo.toml`

Caused by:
  No such file or directory (os error 2)


use network::Network;
use activations::SIGMOID;
use matrix::Matrix;
use std::env;
*/

fn main() {
/*
	env::set_var("RUST_BACKTRACE", "1");
    let inputs = vec![
		vec![0.0, 0.0],
		vec![0.0, 1.0],
		vec![1.0, 0.0],
		vec![1.0, 1.0],
	];
	let targets = vec![vec![0.0], vec![1.0], vec![0.0], vec![1.0]];

    let mut network = Network::new(vec![2,3,1],SIGMOID,0.5);
   
    network.train(inputs, targets, 100000);

	println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 0.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 1.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 0.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 1.0])));
*/

    println!("Hello, world!");
}

