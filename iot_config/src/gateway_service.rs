use crate::GrpcResult;
use helium_proto::services::iot_config::{
    self, GatewayRegionParamsReqV1, GatewayRegionParamsResV1,
};
use tonic::Request;

pub struct GatewayService {}

#[tonic::async_trait]
impl iot_config::Gateway for GatewayService {
    async fn region_params(
        &self,
        _request: Request<GatewayRegionParamsReqV1>,
    ) -> GrpcResult<GatewayRegionParamsResV1> {
        unimplemented!()
    }
}