pub trait Serializable {
    fn serialize(&self) -> String;
}

pub trait Deserializable<T>
where
    T: Sized,
{
    fn deserialize(string: String) -> Option<T>;
}

#[derive(Copy, Clone, Debug)]
pub enum Request {
    Number(u64),
}

impl Serializable for Request {
    fn serialize(&self) -> String {
        match *self {
            Request::Number(n) => n.to_string(),
        }
    }
}

impl Deserializable<Request> for Request {
    fn deserialize(string: String) -> Option<Self> {
        match string.parse::<u64>() {
            Ok(n) => Some(Request::Number(n)),
            _ => None,
        }
    }
}

pub enum Response {
    Time(u64),
    TimeoutError,
}

impl Serializable for Response {
    fn serialize(&self) -> String {
        match *self {
            Response::Time(n) => n.to_string(),
            Response::TimeoutError => String::from("error: timeout"),
        }
    }
}

impl Deserializable<Response> for Response {
    fn deserialize(string: String) -> Option<Self> {
        match string.parse::<u64>() {
            Ok(n) => Some(Response::Time(n)),
            _ => Some(Response::TimeoutError),
        }
    }
}
