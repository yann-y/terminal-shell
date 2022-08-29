
use clap::{Parser, CommandFactory};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// user@ip
    host: Option<String>,
    /// Set port
    #[clap(short, long, value_name = "22")]
    port: Option<u32>,

    // /// Turn debugging information on
    // #[clap(short, long, parse(from_occurrences))]
    // debug: usize,

    // #[clap(subcommand)]
    // command: Option<Commands>,
}

// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[clap(short, long)]
//         list: bool,
//     },
// }
struct Sshinfo {
    username: String,
    host: String,
    port: u32,
}
fn main() {
    let cli = Cli::parse();
    let port:u32 = match cli.port{ 
        Some(s) =>{
            s
        },
        None => 22,
    };
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.host.as_deref() {
        let user = match name.rfind("@") {
            Some(s) => {
                Sshinfo{
                    username:name[0..s].to_string(),
                    host:name[s+1..].to_string(),
                    port:port,
                }
            },
            None => {
                println!("Could not resolve hostname {}: Temporary failure in name resolution",name);
                return ;
            },
        };
        
        println!("Value for name: {}, {},{}",user.username,user.host,user.port);
    }else{
        let mut cmd = Cli::command();
        
        // 输出help
        cmd.print_help().unwrap();
        //println!("{:?}",help)
        // match  cmd.print_help().unwrap(){
        //     Some(helpinfo) =>{
        //         println!("{:#?}",helpinfo)
        //     },
            
        // }
    }
    
    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }
    // println!("Value for name: {}",cli.debug);
    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level app
    // match &cli.command {
    //     Some(Commands::Test { list }) => {
    //         if *list {
    //             println!("Printing testing lists...");
    //         } else {
    //             println!("Not printing testing lists...");
    //         }
    //     }
    //     None => {}
    // }

    // Continued program logic goes here...
}