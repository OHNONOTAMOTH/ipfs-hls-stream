use ipfs_api::*;
use futures_util::stream::*;

pub mod addresses;

#[tokio::main]
pub async fn serve(hash: &str) -> Result<String, String> {
    let client = ipfs_api::IpfsClient::default();

    let gotten = String::from_utf8(client.cat(hash).map_ok(|chunk| chunk.to_vec()).try_concat().await.unwrap()).unwrap();

    Ok(gotten)
}

#[cfg(test)]
mod tests {
    use crate::serve;

    #[test]
    fn ipfscontest() {
        println!("{}", serve("QmRf22bZar3WKmojipms22PkXH1MZGmvsqzQtuSvQE3uhm").unwrap());
    }
}