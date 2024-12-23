use aws_config::{self};
use aws_sdk_sesv2::{config::Credentials, Client as Sesv2Client};

const DEFAULT_AWS_REGION: &str = "ap-northeast-1";

// struct AwsClientPool {
//     sesv2_client: OnceCell<Arc<Sesv2Client>>,
// }

// impl AwsClientPool {
//     async fn get_sesv2_client(&self) -> Arc<Sesv2Client> {
//         self.sesv2_client
//             .get_or_init(|| async {
//                 let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

//                 return Arc::new(Sesv2Client::new(&config));
//             })
//             .clone()
//     }

//     async fn create_sesv2_client(
//         access_key_id: String,
//         secret_access_key: String,
//         // region: Option<String>,
//     ) -> Sesv2Client {
//         // 자격증명 생성
//         let credentials = Credentials::new(
//             access_key_id,
//             secret_access_key,
//             None, // session token (옵션)
//             None, // 만료 시간 (옵션)
//             DEFAULT_AWS_REGION,
//         );

//         // 설정 로드
//         let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
//             .credentials_provider(credentials)
//             .load()
//             .await;

//         // S3 클라이언트 생성
//         Sesv2Client::new(&config)
//     }
// }

pub async fn get_aws_client(access_key_id: &String, secret_access_key: &String) -> Sesv2Client {
    let credentials = Credentials::new(
        access_key_id,
        secret_access_key,
        None, // session token (옵션)
        None, // 만료 시간 (옵션)
        DEFAULT_AWS_REGION,
    );

    // 설정 로드
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(credentials)
        .load()
        .await;

    return aws_sdk_sesv2::Client::new(&config);
}

#[cfg(test)]
mod tests {
    #[test]
    async fn get_aws_client_test() {}
}
