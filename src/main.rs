
use opendal::raw::oio::Read;
use opendal::services::Postgresql;
use opendal::Operator;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let builder = Postgresql::default()
    .root("/")
    .connection_string("postgresql://manjusaka:manjusaka@127.0.0.1:5432/manjusaka")
    .table("data")
    // key field type in the table should be compatible with Rust's &str like text
    .key_field("key")
    // value field type in the table should be compatible with Rust's Vec<u8> like bytea
    .value_field("value");

    let op = Operator::new(builder)?.finish();

    // write data
    op.write("world","abc").await?;

    let result= op.read("world").await?.read_all().await?.to_bytes();
    println!("read data: {:?}", String::from_utf8(result.to_vec()));

    Ok(())
}
