use crate::systems::session::Token as SessionToken;
use juniper::parser::{ParseError, ScalarToken, Token};
use juniper::{ParseScalarResult, Value};

juniper::graphql_scalar!(SessionToken as "SessionToken" where Scalar = <S> {
    description: "A session token in the form of a JSON Web Token (JWT)."

    resolve(&self) -> Value {
        let strrep = self.encode();
        if strrep.is_err() {
            Value::null()
        } else {
            Value::scalar(strrep.unwrap())
        }
    }

    from_input_value(v: &InputValue) -> Option<SessionToken> {
        v.as_scalar_value::<String>()
         .and_then(|s| SessionToken::decode(s).ok())
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        if let ScalarToken::String(value) =  value {
           Ok(S::from(value.to_owned()))
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
});
