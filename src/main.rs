use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: String,

    /// PreBuild or PostBuild
    #[arg(short, long)]
    r#type: String,
}

fn main() {
    let args = Args::parse();
    // read the input file
    let input = std::fs::read_to_string(&args.input).unwrap();
    // create a regex

    let license_regex = regex::Regex::new(r#"RegisterLicense\(.+\)"#).unwrap();
    let jwt_secret_regex = regex::Regex::new(r#"WithSecret\(.+\)"#).unwrap();
    let private_key_regex = regex::Regex::new(r#"private_key = ".+""#).unwrap();

    if args.r#type == "PreBuild" {
        // env variables
        let license = std::env::var("SyncfusionLicenseKey").unwrap_or_default();
        let jwt_secret = std::env::var("MITAMA_AUTH_JWT_SECRET").unwrap_or_default();
        let private_key = std::env::var("GOOGLE_CLOUD_PRIVATE_KEY").unwrap_or_default();
        // replace the license key
        let input = license_regex.replace_all(&input, format!("RegisterLicense(\"{license}\")").as_str());
        // replace the jwt secret
        let input = jwt_secret_regex.replace_all(&input, format!("WithSecret(\"{jwt_secret}\")").as_str());
        // replace the private key
        let input = private_key_regex.replace_all(&input, format!("private_key = \"{private_key}\"").as_str());
        // write the file
        std::fs::write(&args.input, &*input).unwrap();
    } else if args.r#type == "PostBuild" {
        // replace the license key
        let input = license_regex.replace_all(&input, "RegisterLicense(\"SYNCFUSION_LICENSE_KEY\")");
        // replace the jwt secret
        let input = jwt_secret_regex.replace_all(&input, "WithSecret(\"MITAMA_AUTH_JWT_SECRET\")");
        // replace the private key
        let input = private_key_regex.replace_all(&input, "private_key = \"GOOGLE_CLOUD_PRIVATE_KEY\"");
        // write the file
        std::fs::write(&args.input, &*input).unwrap();
    }
}
