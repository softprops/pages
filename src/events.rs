
use hyper::method::Method;
use rustc_serialize::{Encodable,json};
use super::{Acknowledge, EventResponse, PagerDuty, Result, Resolve, Trigger};

const EVENTS: &'static str = "https://events.pagerduty.com/generic/2010-04-15/create_event.json";

pub struct Events<'a> {
    pager: &'a PagerDuty<'a>
}

impl<'a> Events<'a> {
    pub fn new(pager: &'a PagerDuty<'a>) -> Events<'a> {
        Events {
            pager: pager
        }
    }

    pub fn trigger(&self, trigger: &Trigger) -> Result<EventResponse> {
        self.event(trigger)
    }

    pub fn acknowlege<D: Encodable>(&self, ack: &Acknowledge<D>) -> Result<EventResponse> {
        self.event(ack)
    }

    pub fn resolve<D: Encodable>(&self, res: &Resolve<D>) -> Result<EventResponse> {
        self.event(res)
    }

    fn event<T>(&self, event: &T) -> Result<EventResponse> where T: Encodable {
        let data = json::encode(&event).unwrap();
        self.pager.request(
            Method::Post,
            EVENTS,
            Some(data.as_bytes())
        )
    }
}
