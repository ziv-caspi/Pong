use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail, Ok, Result};
use crossbeam::channel::{bounded, Sender};
use uuid::Uuid;

use crate::{runner::messages::PotentialPlayer, utils::events::EventTopic};

const MATCH_TIMEOUT: u64 = 60;

#[derive(Clone)]
pub struct OnNewMatch {
    pub match_id: String,
    pub players: Vec<PotentialPlayer>,
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub nickname: String,
}

struct PendingUser {
    user: User,
    available: bool,
}

struct Match {
    id: String,
    potential_players: Vec<String>,
    ready_players: Vec<String>,
    creation_time: Instant,
    start_time: Option<Instant>,
    started: Sender<()>,
}

impl Match {
    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.started.send(());
    }
}

#[derive(Clone, Debug)]
pub enum OnMatchStatusChange {
    OnTimeout(String),
    OnDeath(String),
    OnStart(OnMatchStart),
    OnReady(String),
}

#[derive(Clone, Debug)]
pub struct OnMatchStart {
    pub match_id: String,
    pub player1: String,
    pub player2: String,
}

#[derive(Clone)]
pub struct DataLayerEvents {
    pub on_new_match: EventTopic<OnNewMatch>,
    pub on_match_change: EventTopic<OnMatchStatusChange>,
}

pub struct MatchmakingDataLayer {
    pub events: DataLayerEvents,

    pending_players: Vec<PendingUser>,
    matches: Vec<Match>,
}

impl MatchmakingDataLayer {
    pub fn new() -> Self {
        Self {
            pending_players: vec![],
            matches: vec![],
            events: DataLayerEvents {
                on_new_match: EventTopic::new(),
                on_match_change: EventTopic::new(),
            },
        }
    }

    pub fn register(&mut self, user: User) -> Result<()> {
        if let Some(_) = self.pending_players.iter().find(|player| player.user.id == user.id) {
            bail!("there is already a player with this id");
        }

        self.pending_players.push(PendingUser { user, available: true });

        println!("registered client, current size: {}", self.pending_players.len());
        self.look_for_matches();

        Ok(())
    }

    pub fn remove_from_queue(&mut self, id: String) -> Result<()> {
        let i = self.player_position_by_id(&id)?;
        self.pending_players.remove(i);
        println!("removed player from queue");
        Ok(())
    }

    pub fn look_for_matches(&mut self) {
        while self.pending_players.len() >= 2 {
            let mut players: Vec<PendingUser> = vec![];
            for _ in 0..2 {
                players.push(self.pending_players.pop().unwrap());
            }

            let potential_players: Vec<PotentialPlayer> = players
                .iter()
                .map(|m| {
                    self.refresh_availabillty(m.user.id.clone(), false);
                    //m.user.id.clone()
                    PotentialPlayer { id: m.user.id.clone(), nickname: m.user.nickname.clone() }
                })
                .collect();
            let ids: Vec<String> = potential_players.iter().map(|p| p.id.clone()).collect();
            let id = self.create_match(&ids);
            self.events
                .on_new_match
                .invoke(OnNewMatch { match_id: id, players: potential_players });
        }
    }

    pub fn mark_player_as_ready(&mut self, match_id: String, player_id: String) -> Result<()> {
        let i = self.match_position_by_id(&match_id)?;
        let m = &mut self.matches[i];
        let mut found = false;
        for player in &m.potential_players {
            if player.to_owned() == player_id {
                found = true;
                break;
            }
        }

        if !found {
            bail!("this player is not a part of this match");
        }

        m.ready_players.push(player_id.clone());
        println!("new ready player, ready players: {}", m.ready_players.len());
        self.events.on_match_change.invoke(OnMatchStatusChange::OnReady(player_id));

        if m.ready_players.len() == m.potential_players.len() {
            println!("starting match!");
            m.start();
            let player1 = &m.ready_players[0];
            let player2 = &m.ready_players[1];
            self.events.on_match_change.invoke(OnMatchStatusChange::OnStart(OnMatchStart {
                match_id: m.id.clone(),
                player1: player1.to_owned(),
                player2: player2.to_owned(),
            }))
        }

        Ok(())
    }

    pub fn remove_from_match(&mut self, match_id: String, player_id: String) -> Result<()> {
        self.delete_match(match_id) // Right now removing player is the same as killing match
    }

    fn create_match(&mut self, potential_players: &[String]) -> String {
        let id = Uuid::new_v4();
        let (s, r) = bounded(1);
        let timeout = self.events.on_match_change.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(MATCH_TIMEOUT));

            // todo: actually kill match
            // match not started
            if r.is_empty() {
                timeout.invoke(OnMatchStatusChange::OnTimeout(id.to_string()))
            }
        });

        let m = Match {
            id: id.to_string(),
            potential_players: potential_players.to_vec(),
            ready_players: vec![],
            creation_time: Instant::now(),
            start_time: None,
            started: s,
        };
        self.matches.push(m);

        id.to_string()
    }

    fn delete_match(&mut self, match_id: String) -> Result<()> {
        let i = self.match_position_by_id(&match_id)?;
        self.matches.remove(i);
        self.events.on_match_change.invoke(OnMatchStatusChange::OnDeath(match_id));
        Ok(())
    }

    fn refresh_availabillty(&mut self, id: String, available: bool) -> Result<()> {
        let i = self.player_position_by_id(&id)?;
        self.pending_players[i].available = available;
        Ok(())
    }

    fn player_position_by_id(&self, id: &str) -> Result<usize> {
        match self.pending_players.iter().position(|player| player.user.id == id) {
            Some(pos) => Ok(pos),
            None => Err(anyhow!("could not find player with this id")),
        }
    }

    fn match_position_by_id(&self, id: &str) -> Result<usize> {
        match self.matches.iter().position(|m| m.id == id) {
            Some(pos) => Ok(pos),
            None => Err(anyhow!("could not find match with this id")),
        }
    }
}
