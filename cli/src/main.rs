use clap::{clap_app, crate_version};

fn main() {
    let clap = clap_app!( cli =>
        (version:crate_version!())
        (author: "Rajesh Kannan")
        (about: "Simple command line example")
        (@arg project_name: +required "What's your project name?")
    )
    .get_matches();

    println!("Project name = {:?}", clap.value_of("project_name"));
    println!("done!");
}
