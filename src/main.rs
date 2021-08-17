extern crate timer;

use crate::CmState::Follower;
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

struct Server {}

// Volatile Raft state on all servers
#[derive(PartialEq, Copy, Clone, Ord, PartialOrd, Eq, Debug, Hash)]
enum CmState {
    Follower,
    Candidate,
    Leader,
    Dead,
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

// State persistent Raft state on all servers.
struct State {
    current_term: i32,
    voted_for: i32,
}

impl State {
    fn default() -> Self {
        State {
            current_term: 0,
            voted_for: 0,
        }
    }
}

// ConsensusModule implements single node of Raft consensus.
struct ConsensusModule {
    id: i32,            // server ID of this CM.
    server: Server,     // server that contains this CM, used to issue RPC calls to peers.
    peer_ids: Vec<i32>, // vec of peers' ids.
    // State persistent Raft state on all servers.
    current_term: i32,
    voted_for: i32,
    // Volatile Raft state on all servers
    state: CmState,
    election_reset_event: SystemTime,
    done: bool,
}

const CM_MAX_DURATION: u64 = 10;

impl ConsensusModule {
    pub fn new(id: i32, peer_ids: Vec<i32>, server: Server) -> Self {
        ConsensusModule {
            id,
            peer_ids,
            server,
            current_term: 0,
            voted_for: 0,
            state: Follower,
            election_reset_event: SystemTime::now(),
            done: false,
        }
    }

    async fn test(self: Rc<ConsensusModule>) {
        task::spawn( async move {
            println!("{}", self.done)
        });
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

        let sm = Arc::new(Mutex::new(self));
        let s_guard = Arc::clone(&sm);
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(10));

            loop {
                interval.tick().await;
                println!("tick");

                let s = s_guard.lock().await;
                if s.state != CmState::Candidate && s.state != CmState::Follower {
                    println!("in election timer state={}, bailing out", s.state);
                    return;
                }

                if term_started != s.current_term {
                    println!(
                        "in election timer term changed from {} to {}, bailing out",
                        term_started, s.current_term
                    );
                    return;
                }

                // Start an election if we haven't heard from a leader or haven't voted for
                // someone for the duration of the timeout.
                if s.election_reset_event.elapsed().unwrap() > timeout_duration {
                    return;
                }
            }
        });
        forever.await;
    }

    fn start_election(&mut self) {
        println!("finish");
    }

    fn election_timeout(&self) -> Duration {
        let mut rng = rand::thread_rng();
        let dur = rng.gen_range(0..CM_MAX_DURATION);
        Duration::new(dur, 0)
    }
}

#[tokio::main]
async fn main() {
    // use std::time::Duration;
    // use tokio::{task, time}; // 1.3.0
    //
    // let forever = task::spawn(async {
    //     let mut counter = 0;
    //     let mut interval = time::interval(Duration::from_secs(1));
    //
    //     loop {
    //         interval.tick().await;
    //         counter += 1;
    //         println!("counter: {}", counter);
    //         if counter == 2 {
    //             return;
    //         }
    //     }
    // });
    // println!("waiting");
    // forever.await;
    //
    // use tokio::time::sleep;
    // let count = Arc::new(Mutex::new(10));
    // let mut c = 0;
    // for i in 0..5 {
    //     let count_guard = Arc::clone(&count);
    //     tokio::spawn(async move {
    //         for _ in 0..10 {
    //             count_guard.lock().await;
    //             c+=1;
    //             println!("{}:{}", i, c);
    //         }
    //     });
    // }
    //
    // loop {z
    //     if *count.lock().await >= 50 {
    //         break;
    //     }
    // }
    // println!("Count hit 50.");

    let mut cm: ConsensusModule = ConsensusModule::new(0, vec![0], Server {});
    Rc::new(cm).test().await;
    println!("finish");
    println!("{}", cm.done);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
