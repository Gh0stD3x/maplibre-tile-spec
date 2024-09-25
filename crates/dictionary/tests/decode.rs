
use dictionary::Dictionary;
use maplibre_tile_spec::StringEncoding;

#[test]
fn test_decode_dictionary() {
    let input = String::from("USA,Mexico,Canada,Argentina:0,0,0,0,1,2,1,1,1,3");
    let expected = String::from("USA,USA,USA,USA,Mexico,Canada,Mexico,Mexico,Mexico,Argentina");
    
    let decoded = Dictionary::decode(&input);
    
    assert_eq!(decoded, expected);
}
