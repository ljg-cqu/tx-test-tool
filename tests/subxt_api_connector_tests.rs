use subxt::PolkadotConfig;
use substrate_txtesttool::subxt_api_connector::connect;

#[tokio::test]
async fn test_connect_local_node() {
    let url = "ws://127.0.0.1:9944";
    let use_legacy_backend = true;
    let result = connect::<PolkadotConfig>(url, use_legacy_backend).await;
    assert!(result.is_ok(), "Failed to connect to local node: {:?}", result.err());
}