use crate::server::{RPCServer, Srv};

use crate::ruftp::{RequestVoteArgs, RequestVoteReply};
use rand::Rng;
use std::borrow::Borrow;
use std::borrow::Cow::Borrowed;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::runtime::{Builder, Handle};
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::sleep;
use tokio::{task, time}; // 1.3.0

#[derive(Debug)]
struct LogEntry {}

// Volatile Raft state on all servers
#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum CmState {
    Follower,
    Candidate,
    Leader,
    Dead,
}

impl Default for CmState {
    fn default() -> Self {
        Self::Follower
    }
}

impl Display for CmState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Follower => "Follower",
            CmState::Candidate => "Candidate",
            CmState::Leader => "Leader",
            CmState::Dead => "Dead",
        };
        write!(f, "{}", desc)
    }
}

// ConsensusModule implements single node of Raft consensus.
pub struct ConsensusModule<S: RPCServer + std::marker::Send> {
    id: i32,            // server ID of this CM.
    peer_ids: Vec<i32>, // vec of peers' id.
    // State persistent Raft state on all server.
    current_term: i32,
    voted_for: i32,
    // Volatile Raft state on all servers
    state: CmState,
    election_reset_event: SystemTime,
    done: bool,
    log: Vec<LogEntry>,
    server: S,
}

const CM_MAX_DURATION: u64 = 10;

impl<S: RPCServer + std::marker::Send> ConsensusModule<S> {
    pub fn new(id: i32, peer_ids: Vec<i32>, server: S) -> Self {
        ConsensusModule {
            id,
            peer_ids,
            current_term: 0,
            voted_for: -1,
            state: CmState::Follower,
            election_reset_event: SystemTime::now(),
            done: false,
            log: Vec::new(),
            server,
        }
    }

    // run runs election timer.
    pub async fn run_election_timer(&mut self) {
        let timeout_duration = self.election_timeout();
        let term_started = self.current_term.clone();

        println!(
            "election time started {:?}, term: {}",
            timeout_duration, term_started
        );

        // This loops until either:
        // - we discover the election timer is no longer needed, or
        // - the election timer expires and this CM becomes a candidate
        // In a follower, this typically keeps running in the background for the
        // duration of the CM's lifetime.
        let mut interval = time::interval(Duration::from_millis(10));

        loop {
            interval.tick().await;
            println!("tick");

            if self.state != CmState::Candidate && self.state != CmState::Follower {
                println!("in election timer state={}, bailing out", self.state);
                return;
            }

            if term_started != self.current_term {
                println!(
                    "in election timer term changed from {} to {}, bailing out",
                    term_started, self.current_term
                );
                return;
            }

            // Start an election if we haven't heard from a leader or haven't voted for
            // someone for the duration of the timeout.
            if self.election_reset_event.elapsed().unwrap() > timeout_duration {
                return;
            }
        }
    }

    fn start_election(&mut self) {
        self.state = CmState::Candidate;
        self.current_term += 1;
        let saved_current_term = self.current_term;
        self.election_reset_event = SystemTime::now();
        self.voted_for = self.id;

        println!(
            "becomes candidate term: {}; log: {:?}",
            saved_current_term, self.log
        );
        let mut votes_received = 1;

        for peer_id in self.peer_ids.iter() {
            let req = RequestVoteArgs {
                term: saved_current_term,
                candidate_id: self.id,
            };

            let reply = self.server.call_vote(peer_id, req).unwrap();
            if self.state != CmState::Candidate {
                println!("while waiting for reply, state {}", self.state);
                return;
            }

            if reply.term > saved_current_term {
                println!("term out of date");
                // self.become_follower(reply.term)
            } else if reply.term == saved_current_term {
                votes_received += 1;
                if votes_received * 2 > self.peer_ids.len() + 1 {
                    // Won the election!
                    // self.state_leader()
                }
            }
        }
        self.run_election_timer();
    }

    pub fn request_vote(
        &mut self,
        req: &RequestVoteArgs,
    ) -> Result<RequestVoteReply, &'static str> {
        if self.state == CmState::Dead {
            return Err("cm is already dead");
        }
        println!(
            "request_vote: [current_term: {}, voted_for: {}]",
            self.current_term, self.voted_for
        );
        let mut reply = RequestVoteReply {
            term: self.current_term,
            vote_granted: false,
        };

        if req.term > self.current_term {
            println!("... term out of date in request_vote")
            // self.becomeFollower(req.term)
        }
        if req.term == self.current_term
            && (self.voted_for == -1 || self.voted_for == req.candidate_id)
        {
            reply.vote_granted = true;
            self.election_reset_event = SystemTime::now()
        }

        Ok(reply)
    }

    async fn start_leader(&mut self) {
        self.state = CmState::Leader;
        println!("becomes leader: term={}", self.current_term);

        let mut interval = time::interval(Duration::from_millis(50));
        loop {
            self.leader_send_heartbeats();
            interval.tick().await;
            if self.state != CmState::Leader {
                return;
            }
        }
    }

    fn leader_send_heartbeats(&self) {
        for peerID in &self.peer_ids {
            println!("appending entries for {} peer", peerID);
        }
    }

    fn election_timeout(&self) -> Duration {
        let mut rng = rand::thread_rng();
        let dur = rng.gen_range(0..CM_MAX_DURATION);
        Duration::new(dur, 0)
    }
}
