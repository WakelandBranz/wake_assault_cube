use crate::cheat::{
    process::Process,
    sdk::player::Player,
};
use super::offsets::{entity_list::*, player::HEALTH};

use std::time::Instant;

use log::{info, debug};

#[derive(Clone, Debug)]
pub struct EntityList {
    mem: Process,
    pub address: u32,
    pub entity_count: u32,
    pub entities: Vec<Player>,
}

impl EntityList {
    pub fn new(mem: Process) -> Self {
        let address = mem.read::<u32>(mem.base_address + ENTITY_LIST).unwrap();
        let entity_count = mem.read::<u32>(mem.base_address + ENTITY_LIST_SIZE).unwrap();
        Self {
            mem,
            address,
            entity_count,
            entities: Vec::with_capacity(MAX_PLAYERS as usize),
        }
    }

    /// Use before updating entity list!
    fn update_entity_count(&mut self) {
        // Read the entity list size
        match self.mem.read::<u32>(self.mem.base_address + ENTITY_LIST_SIZE) {
            Some(size) => self.entity_count = size,
            None => self.entity_count = 0
        }
    }

    /// Clears current entity list
    //fn clear_entity_list(&mut self) {
    //    // Clear by setting all to None
    //    for entity in self.entities.iter_mut() {
    //        *entity = None;
    //    }
    //}

    /// Update entity list
    pub fn update(&mut self) -> Option<()> {
        self.update_entity_count();
        self.entities.clear(); // Reuse existing allocation

        for i in 0..self.entity_count {
            let player_addr = match self.mem.read::<u32>(self.address + (i * 0x4)) {
                Some(addr) => addr,
                None => continue
            };

            // Sometimes players are null
            if player_addr == 0x0 {
                continue
            }

            // Double check to ensure that the entity list is valid.
            let player: Player = match self.mem.read::<Player>(player_addr) {
                Some(player) => player,
                None => {
                    debug!("Entity list likely invalid (could be loading). Resetting...");
                    self.entities.clear();
                    break
                }
            };

            //debug!("Entity {} position: {}", i, player.pos);
            self.entities.push(player)
        }
        Some(())
    }

    /// Get list of entities -- Not currently necessary
    fn _get_entities(&mut self) -> Vec<Player> {
        self.update();
        self.entities.clone()
    }

    // BENCHMARKING ----------------
    pub fn benchmark_updates(&mut self, iterations: u32) {
        let mut times = Vec::with_capacity(iterations as usize);

        for _ in 0..iterations {
            let start = Instant::now();
            self.update();
            times.push(start.elapsed());
        }

        // Calculate statistics
        let total: f64 = times.iter().map(|t| t.as_micros() as f64).sum();
        let avg = total / iterations as f64;
        let min = times.iter().map(|t| t.as_micros()).min().unwrap();
        let max = times.iter().map(|t| t.as_micros()).max().unwrap();

        info!("Entity List Update Benchmarks:");
        info!("Iterations: {}", iterations);
        info!("Average: {:.2} µs", avg);
        info!("Min: {} µs", min);
        info!("Max: {} µs", max);
    }
}