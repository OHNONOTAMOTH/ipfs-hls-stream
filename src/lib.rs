use ipfs_api::*;
use futures_util::stream::*;

pub mod addresses;

#[tokio::main]
pub async fn get(hash: &str) -> Result<addresses::Index, Error> {
    let client = ipfs_api::IpfsClient::default();

    let indextextjson = String::from_utf8(client.cat(hash).map_ok(|chunk| chunk.to_vec()).try_concat().await.unwrap()).unwrap();

    let index = addresses::new(&indextextjson).unwrap();
    Ok(index)
}



#[cfg(test)]
mod tests {
    use crate::get;

    #[test]
    fn ipfscontest() {
        println!("{:?}", get("QmcjtoGz23nxNvEE6gEzNEUwGMMYWknCgPCPHanpjNtHzv").unwrap().title);
    }
}