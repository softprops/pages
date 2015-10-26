use rustc_serialize::Encodable;

#[derive(Debug, RustcDecodable)]
pub struct EventResponse {
    pub status: String,
    pub message: String,
    pub incident_key: String
}

#[derive(Debug, RustcEncodable)]
pub struct Trigger {
    service_key: String,
    event_type: String,
    description: String
}

impl Trigger {
    pub fn new(service_key: &str, description: &str) -> Trigger {
        Trigger {
            service_key: service_key.to_owned(),
            event_type: "trigger".to_owned(),
            description: description.to_owned()
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Acknowledge<D: Encodable> {
    service_key: String,
    event_type: String,
    incident_key: String,
    description: Option<String>,
    details: Option<D>
}

impl<D: Encodable> Acknowledge<D>{
    pub fn new(
        service_key: &str,
        incident_key: &str
    ) -> Acknowledge<D> {
        Acknowledge {
            service_key: service_key.to_owned(),
            event_type: "acknowledge".to_owned(),
            incident_key: incident_key.to_owned(),
            description: None,
            details: None
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Resolve<D: Encodable> {
    service_key: String,
    event_type: String,
    incident_key: String,
    description: Option<String>,
    details: Option<D>
}

impl <D: Encodable> Resolve<D> {
    pub fn new(service_key: &str, incident_key: &str) -> Resolve<D> {
        Resolve {
            service_key: service_key.to_owned(),
            event_type: "resolve".to_owned(),
            incident_key: incident_key.to_owned(),
            description: None,
            details: None
        }
    }
}
