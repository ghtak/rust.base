#![allow(unused)]

use anyhow::Result;


#[tokio::test]
async fn client() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:18080")?;
    hc.do_get("/helloworld").await?.print().await?;
    Ok(())
}