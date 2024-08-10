use camera_system::camera::Camera;
use camera_system::camera_system::CameraSystem;
use common::drone_status::{DroneStatus, TravelLocation};
use common::incident::{Incident, IncidentStatus};
use drone::drone::Drone;
use monitor::monitor::Monitor;

#[test]
fn test_new_incident() {
    let mut monitor = Monitor::new();
    let incident = Incident::new(
        "incident1".to_string(),
        "incident1".to_string(),
        "incident1".to_string(),
        2.0,
        2.0,
        IncidentStatus::Pending,
    );

    // Monitor
    monitor.new_incident(incident.clone());
    assert_eq!(monitor.get_incident(&incident.uuid).unwrap(), &incident);

    // Drone
    let mut drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 5.0);
    assert_eq!(drone.data(), "1;1;3;100");
    drone.add_incident(incident.clone());
    assert_eq!(drone.current_incident().unwrap(), incident);
    drone.set_status(DroneStatus::Travelling(TravelLocation::Incident));
    assert_eq!(
        drone.status(),
        DroneStatus::Travelling(TravelLocation::Incident)
    );
    drone.travel_to(2.0, 2.0);
    assert_eq!(drone.data(), "1.7071067811865475;1.7071067811865475;4;100");

    // Camara
    let mut camera_system = CameraSystem::new();
    let camera = Camera::new(1_u8, 1.5, 1.5, 3.0);
    camera_system.add_camera(camera);
    let camera_data1 = camera_system.cameras_data();
    camera_system.new_incident(incident.clone());
    let camera_data2 = camera_system.cameras_data();
    assert_eq!(camera_data1, "1;1.5;1.5;0");
    assert_eq!(camera_data2, "1;1.5;1.5;1");
}

#[test]
fn test_new_incident_gets_attended() {
    let mut monitor = Monitor::new();
    let incident = Incident::new(
        "incident2".to_string(),
        "incident2".to_string(),
        "incident2".to_string(),
        5.0,
        5.0,
        IncidentStatus::Pending,
    );

    // Monitor
    monitor.new_incident(incident.clone());
    assert_eq!(monitor.get_incident(&incident.uuid).unwrap(), &incident);

    // Drone 1
    let mut drone = Drone::new(1, 1.0, 1.0, 1.0, 1.0, 1.0, 5.0);
    drone.add_incident(incident.clone());
    drone.set_status(DroneStatus::Travelling(TravelLocation::Incident));
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);

    // Drone 2
    let mut drone2 = Drone::new(1, 2.0, 2.0, 1.0, 1.0, 1.0, 5.0);
    drone2.add_incident(incident.clone());
    drone2.set_status(DroneStatus::Travelling(TravelLocation::Incident));
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);

    // Camara
    let mut camera_system = CameraSystem::new();
    let camera = Camera::new(1_u8, 1.5, 1.5, 5.0);
    camera_system.add_camera(camera);
    let camera_data1 = camera_system.cameras_data();
    camera_system.new_incident(incident.clone());
    let camera_data2 = camera_system.cameras_data();
    assert_eq!(camera_data1, "1;1.5;1.5;0");
    assert_eq!(camera_data2, "1;1.5;1.5;1");

    // Drones keeps travelling
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone2.travel_to(incident.x_coordinate, incident.y_coordinate);
    drone.set_status(DroneStatus::AttendingIncident);
    drone2.set_status(DroneStatus::AttendingIncident);
    assert_eq!(drone.data(), "5;5;1;100");
    assert_eq!(drone2.data(), "5;5;1;100");

    monitor.attend_incident(incident.uuid.clone());
    monitor.attend_incident(incident.uuid.clone());
    assert_eq!(
        monitor.get_incident(&incident.uuid).unwrap().status,
        IncidentStatus::InProgress
    );
}
