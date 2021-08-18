use crate::server;

use rand::Rng;
use std::borrow::Borrow;
use std::borrow::Cow::Borrowed;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::runtime::{Builder, Handle};
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::sleep;
use tokio::{task, time}; // 1.3.0

#[derive(Debug)]
struct LogEntry {}

// Volatile Raft state on all servers
#[derive(PartialOrd, PartialEq)]
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
pub struct ConsensusModule {
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
}

const CM_MAX_DURATION: u64 = 10;

impl ConsensusModule {
    pub fn new(id: i32, peer_ids: Vec<i32>) -> Self {
        ConsensusModule {
            id,
            peer_ids,
            current_term: 0,
            voted_for: 0,
            state: CmState::Follower,
            election_reset_event: SystemTime::now(),
            done: false,
            log: Vec::new(),
        }
    }

    pub fn default() -> Self {
        ConsensusModule {
            id: 0,
            peer_ids: vec![],
            current_term: 0,
            voted_for: 0,
            state: CmState::Follower,
            election_reset_event: SystemTime::now(),
            done: false,
            log: vec![],
        }
    }

    // run runs election timer.
    async fn run(&mut self) {
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
            // TODO spawn threads here
            // let response = self.server.call(peer_id, "ConsensusModule.RequestVote".to_string(), args).unwrap();
            if self.state != CmState::Candidate {
                continue;
            }
        }
    }

    fn election_timeout(&self) -> Duration {
        let mut rng = rand::thread_rng();
        let dur = rng.gen_range(0..CM_MAX_DURATION);
        Duration::new(dur, 0)
    }
}
