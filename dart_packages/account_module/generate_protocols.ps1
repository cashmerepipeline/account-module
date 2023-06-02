protoc -I../../protocols    -I../../../cashmere_core/protocols --dart_out=grpc:lib/protocols  ../../protocols/account_service.proto
protoc -I../../protocols    -I../../../cashmere_core/protocols --dart_out=lib/protocols       ../../protocols/account.proto
protoc -I../../protocols    -I../../../cashmere_core/protocols --dart_out=lib/protocols       ../../protocols/login.proto
protoc -I../../protocols    -I../../../cashmere_core/protocols --dart_out=lib/protocols       ../../protocols/password.proto
protoc -I../../protocols    -I../../../cashmere_core/protocols --dart_out=lib/protocols       ../../protocols/account_status.proto