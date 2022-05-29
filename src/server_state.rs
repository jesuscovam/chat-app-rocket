use rocket::{
    form::Form,
    response::stream::{Event, EventStream},
    serde::{Deserialize, Serialize},
    tokio::select,
    tokio::sync::broadcast::{error::RecvError, Sender},
    Shutdown, State,
};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
pub fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send fails if there are no active subscribers.
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}
