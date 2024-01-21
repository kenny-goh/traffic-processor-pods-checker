use std::process::{Command};
use std::{env, io};
use std::thread;
use tablestream::{Column, Stream, col};

/// A struct that we want to print out as a table.
struct Result {
    tenant: String,
    total: u32,
    pending: u32,
    running: u32,
    failed: u32,
    unknown: u32,
    succeeded: u32,
}

impl Result {
    fn new(tenant: String, total: u32, pending: u32, running: u32, failed: u32, unknown: u32, succeeded: u32) -> Result {
        Result {
            tenant,
            total,
            pending,
            running,
            failed,
            unknown,
            succeeded,
        }
    }
}

/// Execute a command and return the output
fn exec(cmd: &str, args: &[&str]) -> String
{
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect(format!("failed to execute command {}", cmd).as_str());

    String::from_utf8(output.stdout).unwrap()
}

/// A simple program that summarizes the status of traffic-processor pods for a list of tenants
/// Usage: cargo run -- <tenant-names in comma separated list>, e.g "foo,bar"
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <tenant-names in comma separated list>, e.g \"foo,bar\"", args[0]);
        return;
    }

    let mut out = io::stdout();
    let mut stream = Stream::new(&mut out, vec![
        // A closure that takes a formatter, and a reference to the result type, and writes it out.
        Column::new(|f, c: &Result| write!(f, "{}", &c.tenant)).header("tenant"),
        col!(Result: .total).header("Total"),
        col!(Result: .pending).header("Pending"),
        col!(Result: .running).header("Running"),
        col!(Result: .failed).header("Failed"),
        col!(Result: .unknown).header("Unknown"),
        col!(Result: .succeeded).header("Succeeded"),
    ]);

    let tenant_names: Vec<&str> = args[1].split(',').collect();

    // Execute the command in parallel for each tenant
    let handles: Vec<_> = tenant_names
        .into_iter()
        .map(|tenant_name| {
            let command = format!(
                r#"
                kubectl -n "traffic-processor" get pods | grep 'traffic-processor-{}.*-intern' |
                awk '/^traffic/ {{total++}} /Pending/ {{pending++}} /Running/ {{running++}} /Failed/ {{failed++}} /Unknown/ {{unknown++}} /Succeeded/ {{succeeded++}} END {{ print "{},", (total ? total: 0), ",", (pending ? pending : 0), ",", (running ? running : 0), ",", (failed ? failed : 0), ",", ( unknown ? unknown : 0), ",", (succeeded ? succeeded : 0) }}'
                "#,
                tenant_name, tenant_name
            );
            thread::spawn(move || exec("sh", &["-c", &command]))
        })
        .collect();

    // Collect the results and print them out
    for handle in handles {
        let result = handle.join().unwrap();
        result.split('\n').for_each(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() == 7 {
                let result = Result::new(
                    fields[0].trim().to_string(),
                    fields[1].trim().parse::<u32>().unwrap(),
                    fields[2].trim().parse::<u32>().unwrap(),
                    fields[3].trim().parse::<u32>().unwrap(),
                    fields[4].trim().parse::<u32>().unwrap(),
                    fields[5].trim().parse::<u32>().unwrap(),
                    fields[6].trim().parse::<u32>().unwrap(),
                );
                stream.row(result).unwrap();
            }
        });
    }

    stream.finish().unwrap();
}
