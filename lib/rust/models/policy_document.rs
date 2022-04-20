use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AuthorizerContext {
  #[serde(rename(serialize = "Set-Cookie"))]
  pub Set_Cookie: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PolicyDocumentStatement {
  pub Action: String,
  pub Effect: String,
  pub Resource: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PolicyDocumentBody {
  pub Version: String,
  pub Statement: Vec<PolicyDocumentStatement>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PolicyDocument {
  pub principalId: String,
  pub policyDocument: PolicyDocumentBody,
  pub context: Option<AuthorizerContext>
}

pub fn generate_document(principal_id: &str, context: Option<AuthorizerContext>, effect: &str, resource: &str) -> PolicyDocument {
  PolicyDocument { 
    principalId: principal_id.to_string(), 
    policyDocument: PolicyDocumentBody {
      Version: "2012-10-17".to_string(),
      Statement: vec![PolicyDocumentStatement {
        Action: "execute-api:Invoke".to_string(),
        Effect: effect.to_string(),
        Resource: resource.to_string()
      }]
    },
    context
  }
}