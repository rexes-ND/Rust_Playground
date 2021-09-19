mod lib;
use lib::LimitTracker;
use lib::MockMessenger;

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.getlen(), 1);
}
