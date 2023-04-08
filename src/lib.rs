use std::io::{StdinLock, StdoutLock, Write};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{de::IoRead, StreamDeserializer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Body {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

type StrIn<'a> = StreamDeserializer<'a, IoRead<StdinLock<'a>>, Message>;

pub struct Node {
    next_msg_id: usize,
    node_id: Option<String>,
    node_ids: Option<Vec<String>>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            next_msg_id: 0,
            node_id: None,
            node_ids: None,
        };
    }

    pub fn init(&mut self, node_id: String, node_ids: Vec<String>) -> Result<()> {
        self.node_id = Some(node_id);
        self.node_ids = Some(node_ids);
        Ok(())
    }

    pub fn run(&mut self, strin: StrIn, outlock: &mut StdoutLock) -> Result<()> {
        for input in strin {
            let input = input.context("STDIN could not be deserialized")?;
            self.handle(input, outlock).context("handler failed")?;
        }
        Ok(())
    }

    pub fn handle(&mut self, msg: Message, outlock: &mut StdoutLock) -> Result<()> {
        match msg.body.payload.clone() {
            Payload::InitOk { .. } => bail!("shouldn't receive init_ok!"),
            Payload::EchoOk { .. } => Ok(()),
            Payload::Echo { echo } => self.reply(outlock, msg, Payload::EchoOk { echo }),
            Payload::Init { node_id, node_ids } => {
                self.init(node_id, node_ids).context("failed to init")?;
                self.reply(outlock, msg, Payload::InitOk)
            }
        }
    }

    fn reply(&mut self, outlock: &mut StdoutLock, msg: Message, payload: Payload) -> Result<()> {
        let reply = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: Some(self.next_msg_id),
                in_reply_to: msg.body.msg_id,
                payload,
            },
        };

        serde_json::to_writer(&mut *outlock, &reply).context("serialize reply")?;
        outlock
            .write_all(b"\n")
            .context("add trailing newline to replies")?;

        self.next_msg_id += 1;
        Ok(())
    }
}
