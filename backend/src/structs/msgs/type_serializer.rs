

#[macro_export]
macro_rules! exact_string {
    ($id:ident, $input:literal) => {
        fn $id<'de, D>(deserializer: D) -> Result<String, D::Error>
        where D: serde::Deserializer<'de> {
            let x = String::deserialize(deserializer)?;
            if(x == $input){
                return Ok(x)
            } else {
                Err(serde::de::Error::custom("hee"))
            }
        }
    };
}

#[test]
fn test() {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    exact_string!(exact, "exact");
    #[derive(Serialize, Deserialize, Debug)]
    struct Test {
        #[serde(deserialize_with="exact")]
        test: String,
    }

    let x = json!({
        "test":"exact"
    });

    let _:Test = serde_json::from_value(x).unwrap();
}
