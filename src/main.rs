extern crate timer;

use crate::CmState::Follower;
use std::time::{SystemTime, Duration};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use std::borrow::Borrow;
use std::borrow::Cow::Borrowed;
use tokio::time::sleep;
use tokio::runtime::{Builder, Handle};
use std::rc::Rc;

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
#[derive(Copy, Clone)]
struct ConsensusModule {
    id: i32,            // server ID of this CM.
    server: Server,     // server that contains this CM, used to issue RPC calls to peers.
    peer_ids: Vec<i32>, // vec of peers' ids.
    // State persistent Raft state on all servers.
    current_term: Arc<Mutex<i32>>,
    voted_for: i32,
    // Volatile Raft state on all servers
    state: Arc<Mutex<CmState>>,
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
            current_term: Arc::new(Mutex::new(0)),
            voted_for: 0,
            state: Arc::new(Mutex::new(Follower)),
            election_reset_event: SystemTime::now(),
            done: false,
        }
    }

    // run runs election timer.
    async fn run(&mut self) {
        let timeout_duration = self.election_timeout();

        let term_guard = self.current_term.lock().await;
        let term_started = term_guard.clone();
        drop(term_guard);

        println!(
            "election time started {:?}, term: {}",
            timeout_duration, term_started
        );

        // This loops until either:
        // - we discover the election timer is no longer needed, or
        // - the election timer expires and this CM becomes a candidate
        // In a follower, this typically keeps running in the background for the
        // duration of the CM's lifetime.

        use tokio::{task, time}; // 1.3.0

        let current_term = Arc::clone(&self.current_term);
        let state = Arc::clone(&self.state);
        let election_reset_event = self.election_reset_event.clone();

        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(10));

            loop {
                interval.tick().await;
                println!("tick");

                let state_guard = state.lock().await;
                let state = state_guard.clone();

                if state != CmState::Candidate && state != CmState::Follower {
                    println!("in election timer state={}, bailing out", state);
                    return;
                }

                let term_guard = current_term.lock().await;
                let current_term = term_guard.clone();
                drop(term_guard);

                if term_started != current_term {
                    println!(
                        "in election timer term changed from {} to {}, bailing out",
                        term_started, current_term
                    );
                    return
                }

                // Start an election if we haven't heard from a leader or haven't voted for
                // someone for the duration of the timeout.
                if election_reset_event.elapsed().unwrap() > timeout_duration {
                    return
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
        let dur = rng.gen_range(0..CM_MAX_DURATION) ;
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

    let mut cm: ConsensusModule = ConsensusModule::new(0, vec![0], Server{});
    cm.run().await;
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
