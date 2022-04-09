mod server;
mod userres;

fn main() {
    userres::main_res1();
    userres::res2::main_res1();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            server::run_server([127, 0, 0, 1], 8080).await;
        });
}
