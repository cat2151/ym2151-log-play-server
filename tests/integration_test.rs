use ym2151_log_player_rust::events::EventLog;

#[test]
fn test_load_simple_fixture() {
    let log =
        EventLog::from_file("tests/fixtures/simple.json").expect("Failed to load simple.json");

    assert_eq!(log.event_count, 3);
    assert_eq!(log.events.len(), 3);
    assert!(log.validate());
}

#[test]
fn test_load_complex_fixture() {
    let log =
        EventLog::from_file("tests/fixtures/complex.json").expect("Failed to load complex.json");

    assert_eq!(log.event_count, 10);
    assert_eq!(log.events.len(), 10);
    assert!(log.validate());

    for event in &log.events {
        assert!(
            event.is_data.is_none(),
            "is_data should be ignored during parsing"
        );
    }
}

#[test]
fn test_load_sample_events_json() {
    let log = EventLog::from_file("sample_events.json").expect("Failed to load sample_events.json");

    assert_eq!(log.event_count, 100, "Event count should be 100");
    assert_eq!(log.events.len(), 100, "Should have 100 events");

    assert!(log.validate(), "Event log should be valid");

    assert_eq!(log.events[0].time, 0);
    assert_eq!(log.events[0].addr, 0x08);
    assert_eq!(log.events[0].data, 0x00);

    assert_eq!(log.events[99].time, 111862);
    assert_eq!(log.events[99].addr, 0x08);
    assert_eq!(log.events[99].data, 0x00);

    for i in 1..log.events.len() {
        assert!(
            log.events[i].time >= log.events[i - 1].time,
            "Events should be sorted by time"
        );
    }
}

#[test]
fn test_load_sample_events_addresses() {
    let log = EventLog::from_file("sample_events.json").expect("Failed to load sample_events.json");

    let addresses: std::collections::HashSet<u8> = log.events.iter().map(|e| e.addr).collect();

    assert!(
        addresses.len() > 1,
        "Should have multiple register addresses"
    );

    assert!(addresses.contains(&0x08), "Should have address 0x08");
    assert!(addresses.contains(&0x20), "Should have address 0x20");
    assert!(addresses.contains(&0x28), "Should have address 0x28");
}

#[test]
fn test_event_time_span() {
    let log = EventLog::from_file("sample_events.json").expect("Failed to load sample_events.json");

    let first_time = log.events[0].time;
    let last_time = log.events[log.events.len() - 1].time;

    assert_eq!(first_time, 0, "First event should be at time 0");
    assert!(
        last_time > 100000,
        "Last event should be well after 100k samples"
    );
}
