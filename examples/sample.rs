use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[clap(flatten)]
    pg: pg_conn::PgConn,
}

fn main() -> Result<()> {
    let args = Args::parse();
    eprintln!("{}", args.pg.get_uri());
    Ok(())
}
