import 'package:account_module/protocols/login.pb.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:flutter/foundation.dart';
import 'package:grpc/grpc.dart';

Future<LoginResponse> login(
  String areaCode,
  String phone,
  String password,
  GrpcCall<LoginRequest, LoginResponse> loginCall,
) async {
  final LoginRequest request = LoginRequest(areaCode: areaCode, phone: phone, password: password);

  try {
    final response = await loginCall(request);
    return response;
  } on GrpcError catch (e) {
    debugPrint(e.toString());
    return Future.error(e);
  }
}
