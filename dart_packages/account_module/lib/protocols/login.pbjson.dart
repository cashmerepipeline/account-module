///
//  Generated code. Do not modify.
//  source: login.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use loginRequestDescriptor instead')
const LoginRequest$json = const {
  '1': 'LoginRequest',
  '2': const [
    const {'1': 'area_code', '3': 1, '4': 1, '5': 9, '10': 'areaCode'},
    const {'1': 'phone', '3': 2, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'password', '3': 3, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `LoginRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginRequestDescriptor = $convert.base64Decode('CgxMb2dpblJlcXVlc3QSGwoJYXJlYV9jb2RlGAEgASgJUghhcmVhQ29kZRIUCgVwaG9uZRgCIAEoCVIFcGhvbmUSGgoIcGFzc3dvcmQYAyABKAlSCHBhc3N3b3Jk');
@$core.Deprecated('Use loginResponseDescriptor instead')
const LoginResponse$json = const {
  '1': 'LoginResponse',
  '2': const [
    const {'1': 'person', '3': 1, '4': 1, '5': 12, '10': 'person'},
    const {'1': 'token', '3': 2, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `LoginResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginResponseDescriptor = $convert.base64Decode('Cg1Mb2dpblJlc3BvbnNlEhYKBnBlcnNvbhgBIAEoDFIGcGVyc29uEhQKBXRva2VuGAIgASgJUgV0b2tlbg==');
@$core.Deprecated('Use logoutRequestDescriptor instead')
const LogoutRequest$json = const {
  '1': 'LogoutRequest',
};

/// Descriptor for `LogoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutRequestDescriptor = $convert.base64Decode('Cg1Mb2dvdXRSZXF1ZXN0');
@$core.Deprecated('Use logoutResponseDescriptor instead')
const LogoutResponse$json = const {
  '1': 'LogoutResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 14, '6': '.account_service.LoginStatus', '10': 'result'},
  ],
};

/// Descriptor for `LogoutResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List logoutResponseDescriptor = $convert.base64Decode('Cg5Mb2dvdXRSZXNwb25zZRI0CgZyZXN1bHQYASABKA4yHC5hY2NvdW50X3NlcnZpY2UuTG9naW5TdGF0dXNSBnJlc3VsdA==');
@$core.Deprecated('Use loginWithValidCodeRequestDescriptor instead')
const LoginWithValidCodeRequest$json = const {
  '1': 'LoginWithValidCodeRequest',
  '2': const [
    const {'1': 'phone', '3': 1, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'valid_code', '3': 2, '4': 1, '5': 9, '10': 'validCode'},
  ],
};

/// Descriptor for `LoginWithValidCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginWithValidCodeRequestDescriptor = $convert.base64Decode('ChlMb2dpbldpdGhWYWxpZENvZGVSZXF1ZXN0EhQKBXBob25lGAEgASgJUgVwaG9uZRIdCgp2YWxpZF9jb2RlGAIgASgJUgl2YWxpZENvZGU=');
@$core.Deprecated('Use loginWithValidCodeResponseDescriptor instead')
const LoginWithValidCodeResponse$json = const {
  '1': 'LoginWithValidCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `LoginWithValidCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginWithValidCodeResponseDescriptor = $convert.base64Decode('ChpMb2dpbldpdGhWYWxpZENvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use getValidateCodeRequestDescriptor instead')
const GetValidateCodeRequest$json = const {
  '1': 'GetValidateCodeRequest',
  '2': const [
    const {'1': 'phone', '3': 1, '4': 1, '5': 9, '10': 'phone'},
  ],
};

/// Descriptor for `GetValidateCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getValidateCodeRequestDescriptor = $convert.base64Decode('ChZHZXRWYWxpZGF0ZUNvZGVSZXF1ZXN0EhQKBXBob25lGAEgASgJUgVwaG9uZQ==');
@$core.Deprecated('Use getValidateCodeResponseDescriptor instead')
const GetValidateCodeResponse$json = const {
  '1': 'GetValidateCodeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `GetValidateCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getValidateCodeResponseDescriptor = $convert.base64Decode('ChdHZXRWYWxpZGF0ZUNvZGVSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
