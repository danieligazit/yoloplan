
use {
    crate::model::{Datasource, Extracted, MusicEvent, EventTime, Person, Piece},
    std::path::Path,
    std::error::Error,
    std::fs,
    tokio_test,
    chrono::prelude::*,
    crate::model::http_client::TestHttpClient,
};


fn read_test_webpage(path: &str) -> Result<String, Box<dyn Error>>{
    let base_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let webpage_path = Path::new(&base_path).join(path);
    Ok(fs::read_to_string(webpage_path)?)
}

#[test]
fn test_extracor() -> Result<(), Box<dyn Error>>{

    let webpage = read_test_webpage("resources/tests/bachtrack_listing")?;
    
    let datasource = super::DS::new(TestHttpClient::new(&webpage));

    let configuration = r#"{"ds_name": "datasource.bachtrack_listing", "value": "/url-something"}"#.as_bytes().to_vec();
    let items = tokio_test::block_on(datasource.extract(&configuration))?;

    assert_eq!(items.len(), 25);
    assert_eq!(items[0], Extracted::MusicEvent(
        MusicEvent{
            artists: vec![
                Person { name: "Bach, Johann Sebastian".to_owned() }, 
                Person { name: "Vivaldi, Antonio".to_owned() }, 
                Person { name: "Handel, George Frideric".to_owned() }, 
                Person { name: "Mozart, Wolfgang Amadeus".to_owned() },
                Person { name: "Beethoven, Ludwig van".to_owned() },
                Person { name: "Haydn, Joseph".to_owned() },
                Person { name: "Schubert, Franz".to_owned() },
            ],
            pieces: vec![],
            description: "Every Thursday and Saturday you can expect a special cultural hallmark in the Munich Residence throughout the year. The Residence Soloists, including members of the Munich Philharmonic Orchestra are performing in the Court Chapel (Hofkapelle), an earlier wedding chapel in which Mozart already performed concerts. You will find weekly changing performances with master-pieces ranging from Bach, Vivaldi, Händel, Haydn and Mozart.".to_owned(),
            time: EventTime{
                start_time: Utc.ymd(2020, 10, 8).and_hms_milli(18, 30, 0, 0).naive_utc(),
                end_time: Utc.ymd(2020, 10, 8).and_hms_milli(20, 30, 0, 0).naive_utc(),
                local_timezone: true,
            }
        })
    );
    Ok(())
}

#[test]
fn test_extracor2() -> Result<(), Box<dyn Error>>{
    let webpage = read_test_webpage("resources/tests/bachtrack_listing2")?;
    
    let datasource = super::DS::new(TestHttpClient::new(&webpage));

    let configuration = r#"{"ds_name": "datasource.bachtrack_listing", "value": "/url-something"}"#.as_bytes().to_vec();
    let items = tokio_test::block_on(datasource.extract(&configuration))?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0], Extracted::MusicEvent(
        MusicEvent{
            artists: vec![
                Person { name: "Mozart, Wolfgang Amadeus".to_owned() }, 
                Person { name: "Handel, George Frideric".to_owned() }
            ],
            pieces: vec![
                Piece { name: "Missa in C, \"Coronation\", K317".to_owned(), artists: vec![Person { name: "Mozart, Wolfgang Amadeus".to_owned() }] }, 
                Piece { name: "Zadok the Priest, HWV 258: God save the King".to_owned(), artists: vec![Person { name: "Handel, George Frideric".to_owned() }] }, 
                Piece { name: "Motet in D, \"Ave verum Corpus\", K618".to_owned(), artists: vec![Person { name: "Mozart, Wolfgang Amadeus".to_owned() }] }, 
                Piece { name: "My heart is inditing; Coronation Anthem No. 3, HWV 261".to_owned(), artists: vec![Person { name: "Handel, George Frideric".to_owned() }] }
            ],
            description: "".to_owned(),
            time: EventTime{
                start_time: Utc.ymd(2020, 10, 23).and_hms_milli(19, 30, 0, 0).naive_utc(),
                end_time: Utc.ymd(2020, 10, 23).and_hms_milli(21, 30, 0, 0).naive_utc(),
                local_timezone: true,
            }
        })
    );
    Ok(())
}

