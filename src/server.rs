use std::{
    collections::{
        HashMap, HashSet
    }
};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room: String
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChangeColor {
    pub id: usize,
    pub color: u32,
    pub room: String
}

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng
}

impl ChatServer {
    pub fn new() -> ChatServer {

        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng()
        }
    }
}

impl ChatServer {
    fn send_message<'r>(&self, room: &'r str, message: &'r str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        let mut r = HashSet::new();
        r.insert(id);

        self.rooms
            .entry(id.to_string())
            .or_insert_with(HashSet::new)
            .insert(id);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone left", 0);
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned())
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone left", 0);
        }

        self.rooms
            .entry(id.to_string())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.send_message(&name, "Someone joined", id);
    }
}

impl Handler<ChangeColor> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChangeColor, ctx: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.room, message, skip_id)
    }
}

// FINISH SENDING IN BSON DATA TO CLIENT
// CLEAN UP CODE TO ONLY REFLECT BSON DATA BEING SENT