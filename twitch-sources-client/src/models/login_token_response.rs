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
pub struct LoginTokenResponse {
    #[serde(rename = "token")]
    pub token: String,
}

impl LoginTokenResponse {
    pub fn new(token: String) -> LoginTokenResponse {
        LoginTokenResponse {
            token,
        }
    }
}


