use ym2151_log_play_server::events::EventLog;

#[test]
fn test_load_simple_fixture() {
    let log =
        EventLog::from_file("tests/fixtures/simple.json").expect("Failed to load simple.json");

        assert_eq!(log.events.len(), 3);
    assert!(log.validate());
}

#[test]
fn test_load_complex_fixture() {
    let log =
        EventLog::from_file("tests/fixtures/complex.json").expect("Failed to load complex.json");

        assert_eq!(log.events.len(), 5); // Updated for current implementation (addr-data pair implementation removed)
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
    let log = EventLog::from_file("output_ym2151.json").expect("Failed to load output_ym2151.json");

    assert_eq!(log.events.len(), 46, "Event count should be 46");
    assert_eq!(log.events.len(), 46, "Should have 46 events");

    assert!(log.validate(), "Event log should be valid");

    assert_eq!(log.events[0].time, 0.0);
    assert_eq!(log.events[0].addr, 0x08);
    assert_eq!(log.events[0].data, 0x00);

    assert_eq!(log.events[45].time, 83895.0 / 55930.0);
    assert_eq!(log.events[45].addr, 0x08);
    assert_eq!(log.events[45].data, 0x00);

    for i in 1..log.events.len() {
        assert!(
            log.events[i].time >= log.events[i - 1].time,
            "Events should be sorted by time"
        );
    }
}

#[test]
fn test_load_sample_events_addresses() {
    let log = EventLog::from_file("output_ym2151.json").expect("Failed to load output_ym2151.json");

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
    let log = EventLog::from_file("output_ym2151.json").expect("Failed to load output_ym2151.json");

    let first_time = log.events[0].time;
    let last_time = log.events[log.events.len() - 1].time;

    assert_eq!(first_time, 0.0, "First event should be at time 0");
    assert!(
        last_time > 50000.0 / 55930.0,
        "Last event should be well after 50k samples"
    );
}
