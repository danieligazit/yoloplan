use {
    crate::model::{Datasource, Extracted},
    std::path::Path,
    std::error::Error,
    std::fs,
    tokio_test,
    crate::model::http_client::TestHttpClient,
};

const DISCOVERY_URL: &str = "https://bachtrack.com/find-concerts/";

fn read_test_webpage() -> Result<String, Box<dyn Error>>{
    let base_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let webpage_path = Path::new(&base_path).join("resources/tests/bachtrack_discovery");
    Ok(fs::read_to_string(webpage_path)?)
}

#[test]
fn test_extracor() -> Result<(), Box<dyn Error>>{
    let webpage = read_test_webpage()?;
    
    let datasource = super::DS::new(TestHttpClient::new(&webpage));

    let configuration = DISCOVERY_URL.as_bytes().to_vec();
    let items = tokio_test::block_on(datasource.extract(&configuration))?;

    assert_eq!(items.len(), 50);
    assert_eq!(items[0], Extracted::Configuration{
        ds_name: "datasource.bachtrack".to_owned(),
        value: "https://bachtrack.com/concert-event/residenz-serenade-munich-residenz-solisten-die-residenz-hofkapelle-5-september-2019/318719".as_bytes().to_vec(),
    });
    Ok(())
}
