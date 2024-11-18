// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_cloud_proto::blockchain::Block as CloudBlock;
use cita_cloud_proto::common::{Hash as CloudHash, HashResponse};
use cita_cloud_proto::evm::rpc_service_server::RpcService;
use cita_cloud_proto::evm::{
    Balance as CloudBalance, BlockNumber, ByteAbi as CloudByteAbi, ByteCode as CloudByteCode,
    ByteQuota as CloudByteQuota, GetAbiRequest, GetBalanceRequest, GetCodeRequest,
    GetStorageAtRequest, GetTransactionCountRequest, Nonce as CloudNonce, Receipt as CloudReceipt,
    ReceiptProof, RootsInfo,
};
use cita_cloud_proto::executor::executor_service_server::ExecutorService;
use cita_cloud_proto::executor::{
    CallRequest as CloudCallRequest, CallResponse as CloudCallResponse,
};
use tonic::{Code, Request, Response, Status};

#[derive(Clone)]
pub struct ExecutorServer {}

#[tonic::async_trait]
impl ExecutorService for ExecutorServer {
    #[instrument(skip_all)]
    async fn exec(&self, request: Request<CloudBlock>) -> Result<Response<HashResponse>, Status> {
        cloud_util::tracer::set_parent(&request);
        let block = request.into_inner();
        debug!("get exec request: {:x?}", block);

        // TODO
        Ok(Response::new(HashResponse::default()))
    }

    #[instrument(skip_all)]
    async fn call(
        &self,
        request: Request<CloudCallRequest>,
    ) -> Result<Response<CloudCallResponse>, Status> {
        cloud_util::tracer::set_parent(&request);

        // TODO
        Ok(Response::new(CloudCallResponse::default()))
    }
}

#[tonic::async_trait]
impl RpcService for ExecutorServer {
    #[instrument(skip_all)]
    async fn get_transaction_receipt(
        &self,
        request: Request<CloudHash>,
    ) -> Result<Response<CloudReceipt>, Status> {
        cloud_util::tracer::set_parent(&request);
        let cloud_hash = request.into_inner();
        debug!("get_transaction_receipt request: {:x?}", cloud_hash);

        // TODO
        Ok(Response::new(CloudReceipt::default()))
    }

    #[instrument(skip_all)]
    async fn get_code(
        &self,
        request: Request<GetCodeRequest>,
    ) -> Result<Response<CloudByteCode>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("get_code request: {:x?}", raw_request);

        if raw_request.address.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Address is none"));
        }

        if raw_request.block_number.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Block number is none"));
        }

        // TODO
        Ok(Response::new(CloudByteCode::default()))
    }

    #[instrument(skip_all)]
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<CloudBalance>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("get_balance request: {:x?}", raw_request);

        if raw_request.address.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Address is none"));
        }

        if raw_request.block_number.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Block number is none"));
        }

        // TODO
        Ok(Response::new(CloudBalance::default()))
    }

    #[instrument(skip_all)]
    async fn get_transaction_count(
        &self,
        request: Request<GetTransactionCountRequest>,
    ) -> Result<Response<CloudNonce>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("get_transaction_count request: {:x?}", raw_request);

        if raw_request.address.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Address is none"));
        }

        if raw_request.block_number.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Block number is none"));
        }

        // TODO
        Ok(Response::new(CloudNonce::default()))
    }

    #[instrument(skip_all)]
    async fn get_abi(
        &self,
        request: Request<GetAbiRequest>,
    ) -> Result<Response<CloudByteAbi>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("get_abi request: {:x?}", raw_request);

        if raw_request.address.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Address is none"));
        }

        if raw_request.block_number.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Block number is none"));
        }

        // TODO
        Ok(Response::new(CloudByteAbi::default()))
    }

    #[instrument(skip_all)]
    async fn estimate_quota(
        &self,
        request: Request<CloudCallRequest>,
    ) -> Result<Response<CloudByteQuota>, Status> {
        cloud_util::tracer::set_parent(&request);
        let call_request = request.into_inner();
        debug!("estimate_quota request: {:x?}", call_request);

        // TODO
        Ok(Response::new(CloudByteQuota::default()))
    }

    #[instrument(skip_all)]
    async fn get_receipt_proof(
        &self,
        request: Request<CloudHash>,
    ) -> Result<Response<ReceiptProof>, Status> {
        let cloud_hash = request.into_inner();
        debug!("get_receipt_proof request: {:x?}", cloud_hash);

        // TODO
        Ok(Response::new(ReceiptProof::default()))
    }

    #[instrument(skip_all)]
    async fn get_roots_info(
        &self,
        request: Request<BlockNumber>,
    ) -> Result<Response<RootsInfo>, Status> {
        let block_number = request.into_inner();
        debug!("get_roots_info request: {:?}", block_number);

        // TODO
        Ok(Response::new(RootsInfo::default()))
    }

    #[instrument(skip_all)]
    async fn get_storage_at(
        &self,
        request: Request<GetStorageAtRequest>,
    ) -> Result<Response<CloudHash>, Status> {
        let raw_request = request.into_inner();
        debug!("get_storage_at request: {:?}", raw_request);

        if raw_request.address.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Address is none"));
        }

        if raw_request.position.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Position is none"));
        }

        if raw_request.block_number.is_none() {
            return Err(Status::new(Code::InvalidArgument, "Block number is none"));
        }

        // TODO
        Ok(Response::new(CloudHash::default()))
    }
}
