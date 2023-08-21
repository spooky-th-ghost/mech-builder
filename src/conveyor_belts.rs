use crate::items::Item;
use bevy::prelude::*;

pub enum Port {
    In(Entity, Item),
    Out(Entity, Item),
}

pub struct Connection(pub Port);

pub struct ConveyorTube {
    input: Connection,
    output: Connection,
}

impl ConveyorTube {
    pub fn update_connection(&mut self, port: Port) {
        match port {
            Port::In(entity, item) => self.output = Connection(Port::Out(entity, item)),
            Port::Out(entity, item) => self.input = Connection(Port::In(entity, item)),
        }
    }
}
