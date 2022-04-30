use super::mio::{Event, Ready};

pub fn assert_event_readiness(event: Event, expected_readiness: Ready) {
  let readiness = event.readiness();
  assert!(
    readiness.contains(expected_readiness),
    "unexpected event with readiness {:?}, expected readiness {:?}",
    readiness,
    expected_readiness,
  );
}
