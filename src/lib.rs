use ipfs_api::*;
use futures_util::stream::*;

pub mod addresses;

#[tokio::main]
pub async fn serve(hash: &str) -> Result<Vec<u8>, Error> {
    let client = ipfs_api::IpfsClient::default();

    let gotten = client.cat(hash).map_ok(|chunk| chunk.to_vec()).try_concat().await.unwrap();

    Ok(gotten)
}

#[cfg(test)]
mod tests {
    use crate::serve;

    #[test]
    fn ipfscontest() {
        println!("{:?}", String::from_utf8(serve("QmRf22bZar3WKmojipms22PkXH1MZGmvsqzQtuSvQE3uhm").unwrap()).unwrap());
    }
}