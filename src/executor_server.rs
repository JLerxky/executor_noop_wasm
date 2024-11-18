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

use std::sync::Arc;

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
use extism::Plugin;
use parking_lot::RwLock;
use prost::Message;
use tonic::{Code, Request, Response, Status};

#[derive(Clone)]
pub struct ExecutorServer {
    pub wasm_runtime: Arc<RwLock<Plugin>>,
}

fn grpc_call_wasm<T, U>(wasm_runtime: Arc<RwLock<Plugin>>, method: &str, input: T) -> U
where
    T: Message,
    U: Message + Default,
{
    let block_raw = T::encode_to_vec(&input);
    let res_raw = wasm_runtime
        .write()
        .call::<Vec<u8>, Vec<u8>>(method, block_raw)
        .unwrap();
    U::decode(&*res_raw).unwrap()
}

#[tonic::async_trait]
impl ExecutorService for ExecutorServer {
    #[instrument(skip_all)]
    async fn exec(&self, request: Request<CloudBlock>) -> Result<Response<HashResponse>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("get exec request: {:x?}", raw_request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "exec",
            raw_request,
        )))
    }

    #[instrument(skip_all)]
    async fn call(
        &self,
        request: Request<CloudCallRequest>,
    ) -> Result<Response<CloudCallResponse>, Status> {
        cloud_util::tracer::set_parent(&request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "call",
            request.into_inner(),
        )))
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
        let raw_request = request.into_inner();
        debug!("get_transaction_receipt request: {:x?}", raw_request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_transaction_receipt",
            raw_request,
        )))
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

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_code",
            raw_request,
        )))
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

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_balance",
            raw_request,
        )))
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

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_transaction_count",
            raw_request,
        )))
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

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_abi",
            raw_request,
        )))
    }

    #[instrument(skip_all)]
    async fn estimate_quota(
        &self,
        request: Request<CloudCallRequest>,
    ) -> Result<Response<CloudByteQuota>, Status> {
        cloud_util::tracer::set_parent(&request);
        let raw_request = request.into_inner();
        debug!("estimate_quota request: {:x?}", raw_request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "estimate_quota",
            raw_request,
        )))
    }

    #[instrument(skip_all)]
    async fn get_receipt_proof(
        &self,
        request: Request<CloudHash>,
    ) -> Result<Response<ReceiptProof>, Status> {
        let raw_request = request.into_inner();
        debug!("get_receipt_proof request: {:x?}", raw_request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_receipt_proof",
            raw_request,
        )))
    }

    #[instrument(skip_all)]
    async fn get_roots_info(
        &self,
        request: Request<BlockNumber>,
    ) -> Result<Response<RootsInfo>, Status> {
        let raw_request = request.into_inner();
        debug!("get_roots_info request: {:?}", raw_request);

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_roots_info",
            raw_request,
        )))
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

        Ok(Response::new(grpc_call_wasm(
            self.wasm_runtime.clone(),
            "get_storage_at",
            raw_request,
        )))
    }
}
