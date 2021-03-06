use rand::{SeedableRng, prelude::StdRng};

use http_clients::{HttpClient, OnionClient, Request};
use session_client::SessionClient;

mod loki;
mod sn_api;
mod ecdh;
mod onions;
mod onions_core;
mod fileserver_api;
mod session_client;
mod tests;
mod node_pool;
mod proof_of_work;
mod swarm_mapping;
mod session_server_client;

mod http_clients;

#[tokio::main]
async fn main() {


    // let token = fileserver_api::get_token(&fileserver_api::DEV_FILESERVER).await.expect("Failed to get token");
    // let token = "";

    // dbg!(&token);

    // return;

    // let mut rng = StdRng::seed_from_u64(0);
    // let network = loki::LOCAL_NET;

    // let pk = loki::PubKey::gen_random(&mut rng, &network);

    // let nodes = sn_api::get_swarm_for_pk(&node_pool[0], &pk.to_string()).await;

    // dbg!(&nodes);


    // fileserver_api::upload_file_via_onion(node_pool.clone(), &fileserver_api::DEV_FILESERVER).await;

    // This is the file that we Audric and I couldn't download from Session Desktop
    // let file = "npoiwi";

    // let mut client = OnionClient::init(&network).await;


    // let res = fileserver_api::get_file_via_onion(&mut client, &fileserver_api::DEV_OPEN_GROUP_SERVER, &token, file).await;

    // dbg!(&res);

    // test_session_clients().await;

    // test_clearnet_requests().await;

    // tests::test_onion_requests().await;
    tests::test_fileserver_requests().await;

}

async fn test_session_clients() {

    // Generate a session identity and send a message to it

    // let alice = SessionClient::new_identity();

    let network = loki::LOCAL_NET;

    let mut rng = StdRng::seed_from_u64(0);

    let pk = loki::PubKey::gen_random(&mut rng, &network);

    let client = SessionClient::new(&network).await;

    let data = vec![1,2,3];

    client.store_message(&pk.to_string(), &data).await;


}

async fn test_clearnet_requests() {
    todo!();
}

async fn test_onion_requests_managed() {
    todo!();
}
