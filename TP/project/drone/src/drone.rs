use std::collections::VecDeque;

use common::drone_status::{DroneStatus, TravelLocation};

use common::incident::Incident;

use crate::utils::Position;

const MINIMUM_BATTERY_LEVEL: usize = 20;
const MAXIMUM_BATTERY_LEVEL: usize = 100;

const BATTERY_DISCHARGE_TRAVELLING: usize = 2;
const BATTERY_DISCHARGE_ATTENDING: usize = 2;
const BATTERY_DISCHARGE_IDLE: usize = 1;
const BATTERY_RECHARGE: usize = 5;

/// Represents a drone
#[derive(Debug, Clone)]
pub struct Drone {
    id: u8,
    position: Position,
    status: DroneStatus,
    battery: usize,
    central: Position,
    anchor: Position,
    current_incident_count: usize,
    incident_queue: VecDeque<Incident>,
    velocity: f64,
    active_range: f64,
}

impl Drone {
    /// Creates a new drone
    pub fn new(
        id: u8,
        x_central: f64,
        y_central: f64,
        x_anchor: f64,
        y_anchor: f64,
        velocity: f64,
        active_range: f64,
    ) -> Self {
        Drone {
            id,
            position: Position::new(x_central, y_central),
            status: DroneStatus::Travelling(TravelLocation::Anchor),
            battery: MAXIMUM_BATTERY_LEVEL,
            central: Position::new(x_central, y_central),
            anchor: Position::new(x_anchor, y_anchor),
            current_incident_count: 0,
            incident_queue: VecDeque::new(),
            velocity,
            active_range,
        }
    }

    /// Returns the data of the drone in string format
    pub fn data(&self) -> String {
        format!(
            "{};{};{};{}",
            self.position.x, self.position.y, self.status, self.battery
        )
    }

    /// Returns the id of the drone
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns true if the battery is below the minimum level
    pub fn is_below_minimun(&self) -> bool {
        self.battery < MINIMUM_BATTERY_LEVEL
    }

    /// Sets the status of the drone
    pub fn set_status(&mut self, status: DroneStatus) {
        self.status = status;
    }

    /// Calculates the distance to a point
    pub fn distance_to(&self, x: f64, y: f64) -> f64 {
        self.position.distance_to(&Position::new(x, y))
    }

    /// Returns the x central coordinate of the drone
    pub fn x_central_coordinate(&self) -> f64 {
        self.central.x
    }

    /// Returns the y central coordinate of the drone
    pub fn y_central_coordinate(&self) -> f64 {
        self.central.y
    }

    /// Returns the x anchor coordinate of the drone
    pub fn x_anchor_coordinate(&self) -> f64 {
        self.anchor.x
    }

    /// Returns the y anchor coordinate of the drone
    pub fn y_anchor_coordinate(&self) -> f64 {
        self.anchor.y
    }

    /// Moves the drone to a point
    pub fn travel_to(&mut self, x: f64, y: f64) {
        let target = Position::new(x, y);
        let distance_to_target = self.position.distance_to(&target);

        if distance_to_target <= self.velocity {
            self.position = target;
        } else {
            self.position.move_towards(&target, self.velocity);
        }
    }

    /// Discharges the battery of the drone
    pub fn discharge_battery(&mut self) {
        let battery_to_discharge = match self.status {
            DroneStatus::Travelling(_) => BATTERY_DISCHARGE_TRAVELLING,
            DroneStatus::Free | DroneStatus::Interrupted => BATTERY_DISCHARGE_IDLE,
            DroneStatus::AttendingIncident => BATTERY_DISCHARGE_ATTENDING,
            DroneStatus::Recharging => {
                return;
            }
        };

        if self.battery < battery_to_discharge {
            self.battery = 0;
            return;
        }

        self.battery -= battery_to_discharge;
    }

    /// Recharges the battery of the drone
    pub fn recharge_battery(&mut self) {
        if self.battery < MAXIMUM_BATTERY_LEVEL {
            self.battery += BATTERY_RECHARGE;
        }
        if self.battery > MAXIMUM_BATTERY_LEVEL {
            self.battery = MAXIMUM_BATTERY_LEVEL;
        }
    }

    /// Returns true if the battery is fully charged
    pub fn is_fully_charged(&self) -> bool {
        self.battery == MAXIMUM_BATTERY_LEVEL
    }

    /// Returns true if the drone is within range of a point
    pub fn is_within_range(&self, x: f64, y: f64) -> bool {
        self.anchor.distance_to(&Position::new(x, y)) < self.active_range
    }

    /// Returns the status of the drone
    pub fn status(&self) -> DroneStatus {
        self.status.clone()
    }

    /// Increments the attending counter of the drone
    pub fn increment_attending_counter(&mut self) {
        self.current_incident_count += 1;
    }

    /// Returns the attending counter of the drone
    pub fn attending_counter(&self) -> usize {
        self.current_incident_count
    }

    /// Returns true if the drone is in the anchor
    pub fn is_in_anchor(&self) -> bool {
        self.position.x == self.anchor.x && self.position.y == self.anchor.y
    }

    /// Returns true if the drone has been interrupted
    pub fn is_interrupted(&self) -> bool {
        self.status == DroneStatus::Interrupted
    }

    /// Adds an incident to the drone queue of incidents
    pub fn add_incident(&mut self, incident: Incident) {
        self.incident_queue.push_back(incident);
    }

    /// Returns the current incident of the drone
    pub fn current_incident(&self) -> Option<Incident> {
        self.incident_queue.front().cloned()
    }

    /// Removes the current incident of the drone
    pub fn remove_current_incident(&mut self) {
        self.incident_queue.pop_front();
        self.current_incident_count = 0;
    }

    /// Checks if the drone is free to attend an incident
    pub fn is_free(&self) -> bool {
        self.status == DroneStatus::Free
            || self.status == DroneStatus::Travelling(TravelLocation::Anchor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drone_data() {
        let drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(drone.data(), "1;1;3;100");
    }

    #[test]
    fn test_travelling_to_central_drone_data() {
        let mut drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);

        drone.set_status(DroneStatus::Travelling(TravelLocation::Central));
        assert_eq!(drone.data(), "1;1;2;100");
    }

    #[test]
    fn test_attending_incident_drone_data() {
        let mut drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);

        drone.set_status(DroneStatus::AttendingIncident);
        assert_eq!(drone.data(), "1;1;1;100");
    }

    #[test]
    fn test_drone_travel_to() {
        let mut drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        drone.travel_to(3.0, 3.0);
        assert_eq!(drone.data(), "1.7071067811865475;1.7071067811865475;3;100");
    }
}
