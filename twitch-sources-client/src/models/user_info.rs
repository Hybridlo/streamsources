/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "username")]
    pub username: String,
}

impl UserInfo {
    pub fn new(username: String) -> UserInfo {
        UserInfo {
            username,
        }
    }
}


