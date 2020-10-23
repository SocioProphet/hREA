---
to: lib/<%= h.changeCase.snake(zome_name) %>/storage/Cargo.toml
---
[package]
name = "hc_zome_<%= h.changeCase.snake(zome_name) %>_storage"
version = "0.1.0"
authors = ["<%=package_author_name%> <<%=package_author_email%>>"]
edition = "2018"

[dependencies]
serde = "1"
# :DUPE: hdk-rust-revid

hdk_graph_helpers = { path = "../../hdk_graph_helpers" }
vf_core = { path = "../../vf_core" }
hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc = { path = "../rpc" }

[lib]
crate-type = ["lib"]
