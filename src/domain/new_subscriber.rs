use crate::domain::subscriber_name::SubscriberName;
use crate::domain::SubscriberEmail;

pub struct NewSubscriber {
    pub(crate) email: SubscriberEmail,
    pub(crate) name: SubscriberName,
}
