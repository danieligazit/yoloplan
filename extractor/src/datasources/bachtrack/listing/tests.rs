
use {
    crate::model::{Datasource, Extracted, MusicEvent, EventTime},
    std::path::Path,
    std::error::Error,
    std::fs,
    tokio_test,
    chrono::prelude::*,
    crate::model::http_client::TestHttpClient,
};

const DISCOVERY_URL: &str = "https://bachtrack.com/concert-event/residenz-serenade-munich-residenz-solisten-die-residenz-hofkapelle-5-september-2019/318719";

fn read_test_webpage() -> Result<String, Box<dyn Error>>{
    let base_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let webpage_path = Path::new(&base_path).join("resources/tests/bachtrack_listing");
    Ok(fs::read_to_string(webpage_path)?)
}

#[test]
fn test_extracor() -> Result<(), Box<dyn Error>>{

    let webpage = read_test_webpage()?;
    
    let datasource = super::DS::new(TestHttpClient::new(&webpage));

    let configuration = DISCOVERY_URL.as_bytes().to_vec();
    let items = tokio_test::block_on(datasource.extract(&configuration))?;

    assert_eq!(items.len(), 25);
    assert_eq!(items[0], Extracted::MusicEvent(
        MusicEvent{
            time: EventTime{
                start_time: Utc.ymd(2020, 10, 8).and_hms_milli(18, 30, 0, 0).naive_utc(),
                end_time: Utc.ymd(2020, 10, 8).and_hms_milli(20, 30, 0, 0).naive_utc(),
                local_timezone: true,
            }
        })
    );
    Ok(())
}
