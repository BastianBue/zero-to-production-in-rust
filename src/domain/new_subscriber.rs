use crate::domain::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    pub(crate) email: String,
    pub(crate) name: SubscriberName,
}
