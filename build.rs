fn main() {
    let url = "https://raw.githubusercontent.com/my-cfd-platform/yourfin-proto-integration-contracts/main/proto/";
    ci_utils::sync_and_build_proto_file(url, "AccountsIntegrationGrpcService.proto");

    let url = "https://raw.githubusercontent.com/my-cfd-platform/proto-files/main/proto/";
    ci_utils::sync_and_build_proto_file(url, "AccountsManagerGrpcService.proto");
}
