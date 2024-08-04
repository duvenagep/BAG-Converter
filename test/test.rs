

#[test]
fn test(){

    let filename = "/Users/paulduvenage/Documents/Rust_Development/Experiments/xml_parse/src/9999NUM08112022-000001.xml"; 
    let content = fs::read_to_string(&filename).expect("something went wrong reading the file");
  
    let loaded: num::BagStand = from_str(&content).unwrap();
    println!("{:?}", loaded);
  
    let reference = num::BagStand {
      stand_bestand: num::StandBestand {
        dataset: "LVBAG".to_string(),
        inhoud: num::Inhoud { 
          gebied: "Nederland".to_string(), 
          leverings_id: "0000000001".to_string(),
          object_typen: num::ObjectTypen {
            object_type: "NUM".to_string()
          }
        },
        stand: vec![num::Stand {
          bag_object: num::BagObject {
            nummeraanduiding: num::Nummeraanduiding { 
              identificatie: "0000200000057534".to_owned(),
              huisnummer: "32".to_string(), 
              huisletter: Some("A".to_string()), 
              huisnummertoevoeging: None,
              postcode: Some("6131BE".to_string()), 
              type_adresseerbaar_object: "Verblijfsobject".to_string(), 
              status: "Naamgeving uitgegeven".to_string(), 
              geconstateerd: "N".to_string(), 
              documentdatum: "2018-03-26".to_string(), 
              documentnummer: "BV05.00043-HLG".to_string(),
              ligtaan: num::LigtAan { openbareruimteref: "1883300000001522".to_string() }
              }
            }
         },
         num::Stand {
          bag_object: num::BagObject {
            nummeraanduiding: num::Nummeraanduiding { 
              identificatie: "0000200000057534".to_string(),
              huisnummer: "32".to_string(), 
              huisletter: Some("A".to_string()),
              huisnummertoevoeging: None, 
              postcode: Some("6131BE".to_string()), 
              type_adresseerbaar_object: "Verblijfsobject".to_string(), 
              status: "Naamgeving ingetrokken".to_string(), 
              geconstateerd: "N".to_string(), 
              documentdatum: "2018-04-04".to_string(), 
              documentnummer: "correctie".to_string(),
              ligtaan: num::LigtAan { openbareruimteref: "1883300000001522".to_string() } 
              }
            }
         }]
      }
    };
  
    assert_eq!(loaded, reference);
}