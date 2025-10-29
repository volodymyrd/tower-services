use crate::concurrency::support;
use tokio_test::{assert_pending, assert_ready, assert_ready_ok};
use tower::limit::concurrency::ConcurrencyLimitLayer;
use tower_test::{assert_request_eq, mock};
use tracing::info;

#[tokio::test(flavor = "current_thread")]
async fn basic_service_limit_functionality_with_poll_ready() {
    let _t = support::trace_init();
    info!("Starting basic_service_limit_functionality_with_poll_ready");
    let limit = ConcurrencyLimitLayer::new(2);
    let (mut service, mut handle) = mock::spawn_layer(limit);

    assert_ready_ok!(service.poll_ready());
    let r1 = service.call("hello 1");

    assert_ready_ok!(service.poll_ready());
    let r2 = service.call("hello 2");

    assert_pending!(service.poll_ready());

    assert!(!service.is_woken());

    // The request gets passed through
    assert_request_eq!(handle, "hello 1").send_response("world 1");

    // The next request gets passed through
    assert_request_eq!(handle, "hello 2").send_response("world 2");

    // There are no more requests
    assert_pending!(handle.poll_request());

    assert_eq!(r1.await.unwrap(), "world 1");

    assert!(service.is_woken());

    // Another request can be sent
    assert_ready_ok!(service.poll_ready());

    let r3 = service.call("hello 3");

    assert_pending!(service.poll_ready());

    assert_eq!(r2.await.unwrap(), "world 2");

    // The request gets passed through
    assert_request_eq!(handle, "hello 3").send_response("world 3");

    assert_eq!(r3.await.unwrap(), "world 3");
}
