cargo run --release --bin sharded-chat-server -- --node_id 4 --remote_listen_addr localhost:34101 --websocket_listen_addr  localhost:34102 --cluster_api_listen_addr  0.0.0.0:34103 --remote_seed_addr localhost:31101 --log_level INFO --metrics_exporter_listen_addr  0.0.0.0:34104