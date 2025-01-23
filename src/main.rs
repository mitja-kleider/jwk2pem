use jsonwebkey::JsonWebKey;
use std::fs;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let jwk_filename = args.filename;
    let jwk_content = fs::read_to_string(jwk_filename).expect("Failed to read JWK file");

    let pem = jwk_to_pem(jwk_content.as_str());
    println!("{}", pem);
}

fn jwk_to_pem(jwk: &str) -> String {
    let jwk: JsonWebKey = jwk.parse().expect("Failed to parse JWK");
    jwk.key.to_pem()
}

#[cfg(test)]
mod tests {
    use crate::jwk_to_pem;

    const EXAMPLE_JWK: &str = include_str!("../examples/rfc7517-jwk.json");
    const EXAMPLE_PEM: &str = include_str!("../examples/rfc7517.pem");

    #[test]
    fn test_jwk_to_pem() {
        assert_eq!(jwk_to_pem(EXAMPLE_JWK), EXAMPLE_PEM);
    }
}
