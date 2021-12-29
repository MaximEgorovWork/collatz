use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    number: u32,
}

fn main() {
    let args = Cli::from_args();

    println!("==== Orig: {} ====", args.number);

    let mut num = args.number;
    let mut count = 0;

    loop {
      if num == 1 {
        break;
      }

      if is_odd(num) {
        num = num * 3 + 1;
      } else {
        num = num / 2;
      }

      count = count + 1;
      println!("Number ({}): {}", count, num);
    }

    println!("==== Count: {} ====", count);
}



fn is_odd(i: u32) -> bool {
    i&1 != 0
}
