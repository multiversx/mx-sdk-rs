use crate::api::{TestApi, TestApiImpl};

use super::UncallableApi;

impl TestApi for UncallableApi {
    type TestApiImpl = Self;

    fn test_api_impl() -> Self::TestApiImpl {
        unreachable!()
    }
}

impl TestApiImpl for UncallableApi {}
