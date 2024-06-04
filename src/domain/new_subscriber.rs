use crate::domain::subscriber_name::SubscriberName;

pub(crate) struct NewSubscriber {
    pub(crate) email: String,
    pub(crate) name: SubscriberName,
}
