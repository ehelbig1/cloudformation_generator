use clap::{App, Arg, SubCommand};
use cloudformation;

fn main() {
    let matches = App::new("Cloudformation Generator")
        .version("1.0")
        .author("Evan Helbig <ehelbig1@yahoo.com>")
        .about("Tool to help generate cloudformation files.")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Generates an empty cloudformation template")
                .takes_value(true)
                .required_unless("file"),
        )
        .arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .value_name("DESCRIPTION")
                .help("Add a description to the cloudformation template")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Read existing template file")
                .takes_value(true)
                .required_unless("output"),
        )
        .subcommand(
            SubCommand::with_name("parameter")
                .about("manage template parameters")
                .version("1.0")
                .author("Evan Helbig <ehelbig1@yahoo.com>")
                .arg(
                    Arg::with_name("add")
                        .short("a")
                        .long("add")
                        .value_name("ADD")
                        .help("Add a parameter to cloudformation template"),
                ),
        )
        .get_matches();

    // let output = matches.value_of("output").unwrap_or("template.yml");
    // let description = matches.value_of("description").unwrap();
    let file = matches.value_of("file").unwrap();

    // cloudformation::Template::new()
    //     .add_description(description)
    //     .add_parameter(
    //         "Test",
    //         cloudformation::Parameter::new(cloudformation::ParameterType::String),
    //     )
    //     .add_resource(
    //         "Test",
    //         cloudformation::Resource::EC2Instance {
    //             properties: Some(
    //                 cloudformation::resource::ec2::EC2InstanceProperties::new()
    //                     .add_instance_type(cloudformation::resource::ec2::InstanceType::T2_Micro),
    //             ),
    //         },
    //     )
    //     .write(output)
    //     .expect("Error writing to file");

    let template = cloudformation::Template::from(file).expect("Error reading file");
    template
        .add_description("This is yet another test")
        .add_resource(
            "MyEC2",
            cloudformation::Resource::EC2Instance {
                properties: Some(
                    cloudformation::resource::ec2::EC2InstanceProperties::new()
                        .add_instance_type(cloudformation::resource::ec2::InstanceType::T2_Micro),
                ),
            },
        )
        .write(file)
        .expect("Error writing file");
}
